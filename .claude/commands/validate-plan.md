

This is the validates the second step in the Connix Spec-Driven Development 
  lifecycle.

1. Parse the plan and details from ./specs/$ARGUMENTS/
2. Read and analyze the feature specification to understand:
   - The feature requirements and user stories
   - Functional and non-functional requirements
   - Success criteria and acceptance criteria
   - Any technical constraints or dependencies mentioned
3. Read the constitution at `/memory/constitution.md` to understand 
  constitutional requirements.

I want you to go through the implementation plan and implementation details, 
looking for areas that could benefit from additional research as claude code and Rust is 
rapidly developing/changing. 

For those areas that you identify that require further research, I want you to 
update the research document with additional details about the specific versions
that we are "going to"/"already do" use in this Connix Platform and spawn 
parallel research tasks to clarify any details using research from the context7 
mcp first, then use your web based tools if needed.

I think we need to break this down into a series of steps.

First, identify a list of tasks that you would need to do during implementation 
that you're not sure of or would benefit from further research.

Write down a list of those tasks. And then for each one of these tasks, I want 
you to spin up a separate research task (launched at the same time) so that the 
net results is we are researching all of those very specific tasks in parallel.
Once you launched the first round of research tasks, I need you to launch a 
second round of tasks that validates the results from the first round.
To make this process more efficient, don't forget to tell them the context of 
the validation task and the requirements of verified results.

Requirements for validated parts of the plan:
- Numerous references to the existing implementation parts/code
- Specific implementation details using library/sdks involved in the planned 
  implementation
- Libraries/SDKs implementation examples exists that is relevant to the planned implementation

Once you have validated all of the tasks, I want you to read through the 
validation results. If you find any areas that you think need further research,
you should relaunch the research tasks for those areas.

Once validation results are satisfactory, I want you to update the implementation
plan with the results of the research.

Researching/searching for rust or claude code hooks in general is not going to help 
us in this case as that is way too untargeted research.

The research needs to help you solve a specific targeted question.
