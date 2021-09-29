# Comparison with Git

TODO: Describe the differences compared to Git here

## Command equivalence table

Note that all `jj` commands can be run on any commit (not just the working copy
commit), but that's left out of the table to keep it simple. For example,
`jj squash -r <revision>` will move the diff from that revision into its parent.

<table>
  <thead>
    <tr>
      <th>Use case</th>
      <th>Jujutsu command</th>
      <th>Git command</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Create a new repo</td>
      <td><code>jj init --git</code> (without <code>--git</code>, you get a
          native Jujutsu repo, which is slow and whose format will change)</td>
      <td><code>git init</code></td>
    </tr>
    <tr>
      <td>Clone an existing repo</td>
      <td><code>jj git clone &lt;source&gt; &lt;destination&gt;</code> (there is no support
          for cloning non-Git repos yet)</td>
      <td><code>git clone &lt;source&gt; &lt;destination&gt;</code></td>
    </tr>
    <tr>
      <td>Update the local repo with all branches from a remote</td>
      <td><code>jj git fetch [--remote &lt;remote&gt;]</code> (there is no
          support for fetching into non-Git repos yet)</td>
      <td><code>git fetch [&lt;remote&gt;]</code></td>
    </tr>
    <tr>
      <td>Update a remote repo with all branches from the local repo</td>
      <td><code>jj git push [--remote &lt;remote&gt;]</code> (there is no
          support for pushing from non-Git repos yet)</td>
      <td><code>git push --all [&lt;remote&gt;]</code></td>
    </tr>
    <tr>
      <td>Update a remote repo with a single branch from the local repo</td>
      <td><code>jj git push --branch &lt;branch name&gt;
                [--remote &lt;remote&gt;]</code> (there is no support for
                pushing from non-Git repos yet)</td>
      <td><code>git push &lt;remote&gt; &lt;branch name&gt;</code></td>
    </tr>
    <tr>
      <td>Show summary of current work and repo status</td>
      <td><code>jj st</code></td>
      <td><code>git status</code></td>
    </tr>
    <tr>
      <td>Show diff of the current change</td>
      <td><code>jj diff</code></td>
      <td><code>git diff HEAD</code></td>
    </tr>
    <tr>
      <td>Show diff of another change</td>
      <td><code>jj diff -r &lt;revision&gt;</code></td>
      <td><code>git diff &lt;revision&gt;^ &lt;revision&gt;</code></td>
    </tr>
    <tr>
      <td>Add a file to the current change</td>
      <td><code>touch filename</code></td>
      <td><code>touch filename; git add filename</code></td>
    </tr>
    <tr>
      <td>Remove a file from the current change</td>
      <td><code>rm filename</code></td>
      <td><code>rm filename</code></td>
    </tr>
    <tr>
      <td>Modify a file in the current change</td>
      <td><code>echo stuff >> filename</code></td>
      <td><code>echo stuff >> filename</code></td>
    </tr>
    <tr>
      <td>Finish work on the current change and start a new change</td>
      <td><code>jj close</code></td>
      <td><code>git commit -a</code></td>
    </tr>
    <tr>
      <td>See log of commits</td>
      <td><code>jj log</code></td>
      <td><code>git log --oneline --graph --decorate</code></td>
    </tr>
    <tr>
      <td>Abandon the current change and start a new change</td>
      <td><code>jj abandon</code></td>
      <td><code>git reset --hard</code> (cannot be undone)</td>
    </tr>
    <tr>
      <td>Make the current change empty</td>
      <td><code>jj restore</code></td>
      <td><code>git reset --hard</code> (same as abandoning a change since Git
          has no concept of a "change")</td>
    </tr>
    <tr>
      <td>Edit description (commit message) of the current change</td>
      <td><code>jj describe</code></td>
      <td>Not supported</td>
    </tr>
    <tr>
      <td>Edit description (commit message) of the previous change</td>
      <td><code>jj describe :@</code></td>
      <td><code>git commit --amend</code> (first make sure that nothing is
          staged)</td>
    </tr>
    <tr>
      <td>Temporarily put away the current change</td>
      <td>Not needed</td>
      <td><code>git stash</code></td>
    </tr>
    <tr>
      <td>Start working on a new change based on the &lt;main&gt; branch</td>
      <td><code>jj co main</code></td>
      <td><code>git checkout -b topic main</code> (may need to stash or commit
          first)</td>
    </tr>
    <tr>
      <td>Move branch A onto branch B</td>
      <td>Not supported yet (can be emulated with
          <code>jj rebase -s</code>)</td>
      <td><code>git rebase B A</code>
          (may need to rebase other descendant branches separately)</td>
    </tr>
    <tr>
      <td>Move change A and its descendants onto change B</td>
      <td><code>jj rebase -s A -d B</code></td>
      <td><code>git rebase --onto B A^ &lt;some descendant branch&gt;</code>
          (may need to rebase other descendant branches separately)</td>
    </tr>
    <tr>
      <td>Reorder changes from A-B-C-D to A-C-B-D</td>
      <td><code>jj rebase -r C -d A; rebase -s B -d C</code> (pass change ids,
          not commit ids, to not have to look up commit id of rewritten C)</td>
      <td><code>git rebase -i A</code></td>
    </tr>
    <tr>
      <td>Move the diff in the current change into the parent change</td>
      <td><code>jj squash</code></td>
      <td><code>git commit --amend -a</code></td>
    </tr>
    <tr>
      <td>Interactively move part of the diff in the current change into the
          parent change</td>
      <td><code>jj squash -i</code></td>
      <td><code>git add -p; git commit --amend</code></td>
    </tr>
    <tr>
      <td>Interactively split a change in two</td>
      <td><code>jj split -r &lt;revision&gt;</code></td>
      <td>Not supported (can be emulated with the "edit" action in
          <code>git rebase -i</code>)</td>
    </tr>
    <tr>
      <td>Interactively edit the diff in a given change</td>
      <td><code>jj edit -r &lt;revision&gt;</code></td>
      <td>Not supported (can be emulated with the "edit" action in
          <code>git rebase -i</code>)</td>
    </tr>
    <tr>
      <td>Resolve conflicts and continue interrupted operation</td>
      <td><code>echo resolved > filename; jj squash</code> (operations don't
          get interrupted, so no need to continue)</td>
      <td><code>echo resolved > filename; git add filename; git
          rebase/merge/cherry-pick --continue</code></td>
    </tr>
    <tr>
      <td>List branches</td>
      <td><code>jj branches</code></td>
      <td><code>git branch</code></td>
    </tr>
    <tr>
      <td>Create a branch</td>
      <td><code>jj branch &lt;name&gt; -r &lt;revision&gt;</code></td>
      <td><code>git branch &lt;name&gt; &lt;revision&gt;</code></td>
    </tr>
    <tr>
      <td>Move a branch forward</td>
      <td><code>jj branch &lt;name&gt; -r &lt;revision&gt;</code></td>
      <td><code>git branch -f &lt;name&gt; &lt;revision&gt;</code></td>
    </tr>
    <tr>
      <td>Move a branch backward or sideways</td>
      <td><code>jj branch &lt;name&gt; -r &lt;revision&gt; --allow-backwards</code></td>
      <td><code>git branch -f &lt;name&gt; &lt;revision&gt;</code></td>
    </tr>
    <tr>
      <td>Delete a branch</td>
      <td><code>jj branch --delete &lt;name&gt; </code></td>
      <td><code>git branch --delete &lt;name&gt;</code></td>
    </tr>
    <tr>
      <td>See log of operations performed on the repo</td>
      <td><code>jj op log</code></td>
      <td>Not supported</td>
    </tr>
    <tr>
      <td>Undo an earlier operation</td>
      <td><code>jj op undo -o &lt;operation id&gt;</code></td>
      <td>Not supported</td>
    </tr>
  </tbody>
</table>