# Syntax (WIP)

**⚠ This syntax is very early and could change**

When rendering it looks for a suffix `$`,
this tells the renderer to not treat it like normal text,
you can escape it by `\$`

the general flow is `$<statement>: <text here> $end`

`$end` is optional if it's a single line, but it's required if it's multi-line like

```text
$<statement>: <- whitespace gets ignored, treat this as a block
<text here> 
<more text here>
$end
```

<details>
  <summary>comments</summary>

```text
$// comments

$//
multi
line
comment
//$

a $// in line comment //$ b
```

</details>

<details>
  <summary>variables</summary>

Variables are pretty straightforward

`title` = `Stuff`

`description` = `Lots of stuff`

```text
# $title
$description
```

**Outputs**:

```text
# Stuff
Lots of stuff
```

</details>

<details>
  <summary>conditionals</summary>

`item` = `stuff`

```text
line 1
line 2 $if item:
$item
$else:
Item doesn't exist
$end
line 2
```

Can be used on a single line, no need for `$end`

```text
line 1
line 2 $if item: $item $else: Item doesn't exist
line 3
```

**true**:

```text
line 1
line 2 stuff
line 3
```

**false**:

```text
line 1
line 2 Item doesn't exist
line 3
```

ignores this line if false

```text
$if item: $item
```

</details>

<details>
  <summary>for loops</summary>

`items`:

```json
[
  { "name": "Stuff 1", "url": "localhost/stuff/1" },
  { "name": "Stuff 2", "url": "localhost/stuff/2" },
  { "name": "Stuff 3", "url": "localhost/stuff/3" }
]
```

```text
$for item in items:
- [$item.name]($item.url)
$end
```

**Outputs**:

```text
- [Stuff 1](localhost/stuff/1)
- [Stuff 2](localhost/stuff/2)
- [Stuff 3](localhost/stuff/3)
```

`nums`: `[2, 3, 4]`

```text
1 $for n in nums: $n $end 5
1 $for n in nums: $n
```

**Outputs**:

```text
1 2 3 4 5
1 2 3 4
```

</details>
