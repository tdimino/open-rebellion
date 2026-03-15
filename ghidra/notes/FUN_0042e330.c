
void __thiscall FUN_0042e330(void *this,int param_1,int param_2)

{
  undefined4 uVar1;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c888;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  FUN_0042dbe0(&local_10);
  *(int *)((int)this + 0x1c) = param_1;
  local_4 = 0;
  uVar1 = FUN_0042e960(this,(uint *)&local_10,param_2,param_1,0x2000,0x2000,(void *)0x1);
  *(undefined4 *)((int)this + 0x24) = uVar1;
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

