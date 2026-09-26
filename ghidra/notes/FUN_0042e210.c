
void __thiscall FUN_0042e210(void *this,int param_1,int param_2)

{
  undefined4 uVar1;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c868;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  FUN_0042dbe0(&local_10);
  *(int *)((int)this + 0x18) = param_1;
  local_4 = 0;
  uVar1 = FUN_0042e960(this,(uint *)&local_10,param_2,param_1,0x200000,0x400,(void *)0x1);
  *(undefined4 *)((int)this + 0x28) = uVar1;
  uVar1 = FUN_0042e960(this,(uint *)&local_10,param_2,*(int *)((int)this + 0x18),1,0x200,(void *)0x1
                      );
  *(undefined4 *)((int)this + 0x2c) = uVar1;
  uVar1 = FUN_0042e960(this,(uint *)&local_10,param_2,*(int *)((int)this + 0x18),2,0x400,(void *)0x1
                      );
  *(undefined4 *)((int)this + 0x30) = uVar1;
  uVar1 = FUN_0042e960(this,(uint *)&local_10,param_2,*(int *)((int)this + 0x18),0x100000,0x80,
                       (void *)0x1);
  *(undefined4 *)((int)this + 0x34) = uVar1;
  uVar1 = FUN_0042e960(this,(uint *)&local_10,param_2,*(int *)((int)this + 0x18),0x100000,0x100,
                       (void *)0x1);
  *(undefined4 *)((int)this + 0x38) = uVar1;
  uVar1 = FUN_0042e960(this,(uint *)&local_10,param_2,*(int *)((int)this + 0x18),0x800,0x800,
                       (void *)0x1);
  *(undefined4 *)((int)this + 0x3c) = uVar1;
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

