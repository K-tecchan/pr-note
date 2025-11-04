Release {{ now() | date(format="%Y-%m-%d") }}

{% for pr in prs %}
  {%- if pr.body is containing("- [x] checked") -%}
    - [x] #{{ pr.number }} by @{{ pr.author }}
  {%- else -%}
    - [ ] #{{ pr.number }} by @{{ pr.author }}
  {%- endif %}
{% endfor %}
