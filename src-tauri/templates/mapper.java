package {{ package.Mapper }};

import {{ package.Entity }}.{{ entity }};
import {{ superMapperClassPackage }};
{% if mapperAnnotationClass -%}
import {{ mapperAnnotationClass }};
{%- endif %}

/**
 * <p>
 * {{ table.comment }} Mapper 接口
 * </p>
 *
 * @author {{ author }}
 * @since {{ date }}
 */
{% if mapperAnnotationClass -%}
@{{ mapperAnnotationName }}
{% endif -%}
{% if kotlin -%}
interface {{ table.mapperName }} : {{ superMapperClass }}<{{ entity }}>
{% else -%}
public interface {{ table.mapperName }} extends {{ superMapperClass }}<{{ entity }}> {

}
{%- endif %}
