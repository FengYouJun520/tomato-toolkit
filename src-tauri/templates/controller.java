package {{ package.Controller }};

import org.springframework.web.bind.annotation.RequestMapping;
{% if restControllerStyle -%}
import org.springframework.web.bind.annotation.RestController;
{% else -%}
import org.springframework.stereotype.Controller;
{%- endif %}
{% if superControllerClassPackage -%}
import {{ superControllerClassPackage }};
{% endif -%}

/**
 * <p>
 * {{ table.comment }} 前端控制器
 * </p>
 *
 * @author {{ author }}
 * @since {{ date }}
 */
{% if restControllerStyle -%}
@RestController
{% else -%}
@Controller
{% endif -%}
@RequestMapping("{% if package.ModuleName and package.ModuleName != "" %}/{{ package.ModuleName }}{% endif %}/{% if controllerMappingHyphenStyle %}{{ controllerMappingHyphen }}{% else %}{{ table.entityPath }}{% endif %}")
{% if kotlin -%}
class {{ table.controllerName }}{% if superControllerClass %} : {{ superControllerClass }}(){% endif -%}
{% else -%}
{% if superControllerClass -%}
public class {{ table.controllerName }} extends {{ superControllerClass }} {
{% else -%}
public class {{ table.controllerName }} {
{%- endif %}

}
{% endif %}