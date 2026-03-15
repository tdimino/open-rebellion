
void __thiscall FUN_00487740(void *this,int *param_1,int param_2)

{
  int iVar1;
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634a58;
  local_c = ExceptionList;
  if (param_1 != (int *)0x0) {
    ExceptionList = &local_c;
    FUN_00520570(local_14);
    local_4 = 0;
    iVar1 = (**(code **)(*param_1 + 0x1c))(local_14);
    if (iVar1 == 0) {
      FUN_00487c90(this,param_1,local_14);
    }
    else if ((param_2 == 0) && (iVar1 = FUN_00487cc0(this,param_1), iVar1 != 0)) {
      FUN_0048a340((void *)((int)this + 0x6c),param_1);
    }
    else {
      FUN_00488030(this,param_1);
      FUN_0048aa90(this,param_1);
      FUN_00439cb0(*(int *)((int)this + 0xc0));
      FUN_0043a2a0(*(void **)((int)this + 0xc0),(int)param_1);
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    ExceptionList = local_c;
    return;
  }
  return;
}

