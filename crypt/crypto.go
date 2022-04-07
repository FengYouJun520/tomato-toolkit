package crypt

import (
	"crypto/hmac"
	"crypto/md5"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/sha512"
	"crypto/x509"
	"encoding/base64"
	"encoding/hex"
	"encoding/pem"
	"errors"
	"fmt"
	"hash"

	"golang.org/x/crypto/bcrypt"
)

type CryptConfig struct {
	Typ        string `json:"typ"`
	Cost       string `json:"cost"`
	Source     string `json:"source"`
	PublicKey  string `json:"publicKey"`
	PrivateKey string `json:"privateKey"`
}

// Crypt 加解密
type Crypt struct {
}

func NewCrypt() *Crypt {
	return &Crypt{}
}

func (c *Crypt) Encode(cryptConfig *CryptConfig) (string, error) {
	switch cryptConfig.Typ {
	case "md5", "sha1", "sha256", "sha512":
		return c.byHmac(cryptConfig)
	case "rsa":
		return c.byRsa(cryptConfig)
	case "bcrypt":
		return c.byBcrypt(cryptConfig)
	case "base64":
		return c.byBase64(cryptConfig)
	default:
		return "", fmt.Errorf("不支持的加密类型: %v", cryptConfig.Typ)
	}
}

func (c *Crypt) byHmac(cryptConfig *CryptConfig) (string, error) {
	var h hash.Hash
	switch cryptConfig.Typ {
	case "md5":
		h = hmac.New(md5.New, []byte(cryptConfig.Cost))
	case "sha1":
		h = hmac.New(sha1.New, []byte(cryptConfig.Cost))
	case "sha256":
		h = hmac.New(sha256.New, []byte(cryptConfig.Cost))
	case "sha512":
		h = hmac.New(sha512.New, []byte(cryptConfig.Cost))
	default:
		return "", fmt.Errorf("不支持的加密类型: %v", cryptConfig.Typ)
	}

	h.Write([]byte(cryptConfig.Source))
	return hex.EncodeToString(h.Sum(nil)), nil
}

func (c *Crypt) byRsa(cryptConfig *CryptConfig) (string, error) {
	block, _ := pem.Decode([]byte(cryptConfig.PublicKey))
	if block == nil {
		return "", errors.New("public key error")
	}
	// 解析公钥
	pubInterface, err := x509.ParsePKIXPublicKey(block.Bytes)
	if err != nil {
		return "", err
	}
	// 类型断言
	pub := pubInterface.(*rsa.PublicKey)
	//加密
	dest, err := rsa.EncryptPKCS1v15(rand.Reader, pub, []byte(cryptConfig.Source))
	if err != nil {
		return "", nil
	}

	return hex.EncodeToString(dest), nil
}

func (c *Crypt) byBcrypt(cryptConfig *CryptConfig) (string, error) {
	encodePass, err := bcrypt.GenerateFromPassword([]byte(cryptConfig.Source), bcrypt.DefaultCost)
	if err != nil {
		return "", err
	}

	return string(encodePass), nil
}

func (c *Crypt) byBase64(cryptConfig *CryptConfig) (string, error) {
	return base64.StdEncoding.EncodeToString([]byte(cryptConfig.Source)), nil
}
