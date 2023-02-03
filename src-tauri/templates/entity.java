package {{ package.Entity }};

{% for pkg in table.importPackages -%}
import {{ pkg }};
{% endfor -%}
{% if springdoc -%}
import io.swagger.v3.oas.annotations.media.Schema;
{% elif swagger -%}
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
{% endif -%}
{% if entityLombokModel -%}
import lombok.Getter;
import lombok.Setter;
    {% if chainModel -%}
import lombok.experimental.Accessors;
    {% endif %}
{% endif %}
/**
 * <p>
 * {{ table.comment }}
 * </p>
 *
 * @author {{ author }}
 * @since {{ date }}
 */
{%- if entityLombokModel %}
@Getter
@Setter
    {%- if chainModel %}
@Accessors(chain = true)
    {%- endif -%}
{%- endif %}
{%- if table.convert %}
@TableName("{{ schemaName }}{{ table.name }}")
{%- endif %}
{%- if springdoc %}
@Schema(name = "{{ entity }}", description = "{{ table.comment }}")
{%- elif swagger %}
@ApiModel(value = "{{ entity }}对象", description = "{{ table.comment }}")
{%- endif -%}
{%- if superEntityClass %}
public class {{ entity }} extends {{ superEntityClass }}{% if activeRecord %}<{{ entity }}>{% endif %} {
{%- elif activeRecord %}
public class {{ entity }} extends Model<{{ entity }}> {
{%- elif entitySerialVersionUid %}
public class {{ entity }} implements Serializable {
{%- else %}
public class {{ entity }} {
{% endif -%}
{% if entitySerialVersionUid %}

    private static final long serialVersionUID = 1L;
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
    {%- if field.keyFlag %}
        {#- 主键 -#}
        {%- if field.keyIdentityFlag %}
    @TableId(value = "{{ field.annotationColumnName }}", type = IdType.AUTO)
        {%- elif idType %}
    @TableId(value = "{{ field.annotationColumnName }}", type = IdType.{{ idType }})
        {%- elif field.convert %}
    @TableId("{{ field.annotationColumnName }}")
        {% endif -%}
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
    {%- endif %}
    private {{ field.columnType[0] }} {{ field.propertyName }};
{% endfor -%}
{#- END 字段循环遍历 -#}

{% if not entityLombokModel -%}
    {% for field in table.fields -%}
        {% if field.columnType[0] == "boolean" or field.columnType[0] == "Boolean" -%}
          {% set getprefix = "is" -%}
        {% else -%}
          {% set getprefix = "get" -%}
        {%- endif %}
    public {{ field.columnType[0] }} {{ getprefix }}{{ field.capitalName }}() {
        return {{ field.propertyName }};
    }
    {% if chainModel %}
    public {{ entity }} set{{ field.capitalName }}({{ field.columnType[0] }} {{ field.propertyName }}) {
    {%- else %}
    public void set{{ field.capitalName }}({{ field.columnType[0] }} {{ field.propertyName }}) {
    {%- endif %}
        this.{{ field.propertyName }} = {{ field.propertyName }};
        {%- if chainModel %}
        return this;
        {%- endif %}
    }
    {% endfor -%}
{% endif -%}
{%- if entityColumnConstant %}
    {%- for field in table.fields %}
    public static final String {{ field.name | upper }} = "{{ field.name }}";
    {%- endfor -%}
{% endif -%}
{%- if activeRecord %}

    @Override
    public Serializable pkVal() {
    {%- if keyPropertyName | define %}
        return this.{{ keyPropertyName }};
    {%- else %}
        return null;
    {%- endif %}
    }
{% endif -%}
{%- if not entityLombokModel %}

    @Override
    public String toString() {
        return "{{ entity }}{" +
    {%- for field in table.fields %}
        {%- if loop.first %}
            "{{ field.propertyName }} = " + {{ field.propertyName }} +
        {%- else %}
            ", {{ field.propertyName }} = " + {{ field.propertyName }} +
        {%- endif %}
    {%- endfor %}
        "}";
    }
{% endif -%}
}
