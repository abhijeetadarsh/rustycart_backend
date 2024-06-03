**Set the configuration:** Once you're in the correct directory, run the commands without the `--global` flag:

```bash
git config user.email "ad.abhijeetadarsh@gmail.com"
git config user.name "abhijeetadarsh"
```

This will set your Git user name and email for this specific repository only. It won't affect the configuration for other repositories on your system.

**Important Note:**

- These configuration settings are stored in a hidden file called `.git/config` within the Git repository folder. You don't need to directly edit this file, as the `git config` command manages it for you.

Now, when you commit changes to this repository, the commits will be associated with the user name and email address you specified.
