To strip everything but the numbers from a file.

```sh
for file in *; do
  mv $file "$(echo $file | sed 's/[^0-9]*//g')";
done
```
