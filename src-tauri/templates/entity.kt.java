package {{ package.Entity }}

{% for pkg in table.importPackages -%}
import {{ pkg }}
{% endfor -%}
{% if springdoc -%}
import io.swagger.v3.oas.annotations.media.Schema;
{% elif swagger -%}
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
{%- endif %}

/**
 * <p>
 * {{ table.comment }}
 * </p>
 *
 * @author {{ author }}
 * @since {{ date }}
 */
{% if table.convert -%}
@TableName("{{ schemaName }}{{ table.name }}")
{% endif -%}
{% if springdoc -%}
@Schema(name = "{{ entity }}", description = "{{ table.comment }}")
{% elif swagger -%}
@ApiModel(value = "{{ entity }}对象", description = "{{ table.comment }}")
{% endif -%}
{% if superEntityClass -%}
class {{ entity }} : {{ superEntityClass }}{% if activeRecord %}<{{ entity }}>{% endif %} {
{% elif activeRecord -%}
class {{ entity }} : Model{{ entity }}() {
{% elif entitySerialVersionUid -%}
class {{ entity }} : Serializable {
{% else -%}
class {{ entity}} {
{% endif -%}

{#- BEGIN 字段循环遍历 -#}
{%- for field in table.fields -%}
    {% if field.keyFlag -%}
        {% set_global keyPropertyName = field.propertyName -%}
    {% endif -%}

    {%- if field.comment and field.comment | length > 0 -%}
        {%- if springdoc %}
    @Schema(description = "{{ field.comment }}")
        {%- elif swagger %}
    @ApiModelProperty("{{ field.comment }}")
        {%- else %}
    /**
     * {{ field.comment }}
     */
        {%- endif %}
    {%- endif -%}

{%- if field.keyFlag -%}
    {#- 主键 -#}
    {%- if field.keyIdentityFlag %}
    @TableId(value = "{{ field.annotationColumnName }}", type = IdType.AUTO)
    {%- elif idType %}
    @TableId(value = "{{ field.annotationColumnName }}", type = IdType.{{ idType }})
    {%- elif field.convert %}
    @TableId("{{ field.annotationColumnName }}")
    {%- endif -%}

{#- 普通字段 -#}
{% elif field.fill -%}
    {#- 存在字段填充设置 -#}
    {%- if field.convert %}
    @TableField(value = "{{ field.annotationColumnName }}", fill = FieldFill.{{ field.fill }})
    {%- else %}
    @TableField(fill = FieldFill.{{ field.fill }})
    {% endif -%}
{%- elif field.convert %}
    @TableField("{{ field.annotationColumnName }}")
{%- endif -%}

{#- 乐观锁注解 -#}
{%- if field.versionField %}
    @Version
{%- endif -%}

{#- 逻辑删除注解 -#}
{%- if field.logicDeleteField %}
    @TableLogic
{%- endif -%}

{%- if field.columnType[0] == "Integer" %}
    var {{ field.propertyName }}: Int? = null
{%- else %}
    var {{ field.propertyName }}: {{ field.columnType[0] }}? = null
{%- endif %}
{% endfor -%}
{#- END 字段循环遍历 -#}

{%- if entityColumnConstant %}
    companion object {
{%- for field in table.fields %}

        const val {{ field.name | upper }} : String = "{{ field.name }}"

{%- endfor %}
    }

{%- endif -%}
{%- if activeRecord %}
    override fun pkVal(): Serializable? {
{%- if keyPropertyName | define %}
        return {{ keyPropertyName }}
{%- else %}
        return null
{%- endif %}
    }
{%- endif %}
    override fun toString(): String {
        return "{{ entity }}{" +
{%- for field in table.fields -%}
{%- if loop.first %}
        "{{ field.propertyName }}=" + {{ field.propertyName }} +
{%- else %}
        ", {{ field.propertyName }}=" + {{ field.propertyName }} +
{%- endif -%}
{%- endfor %}
        "}"
    }
}
