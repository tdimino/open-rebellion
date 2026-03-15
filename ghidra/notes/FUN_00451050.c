
void __fastcall FUN_00451050(int param_1)

{
  void *this;
  int *this_00;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062fc3b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_006011f0(param_1);
  this = (void *)FUN_00618b70(0x118);
  local_4 = 0;
  if (this == (void *)0x0) {
    this_00 = (int *)0x0;
  }
  else {
    this_00 = (int *)FUN_0049ee20(this,*(undefined4 *)(param_1 + 0x1c),*(int *)(param_1 + 0x118),7);
  }
  local_4 = 0xffffffff;
  if (this_00 != (int *)0x0) {
    FUN_005ffce0(this_00,0);
    (**(code **)(*this_00 + 0x2c))(5);
    FUN_005f4f10((void *)(*(int *)(param_1 + 0x118) + 0x6c),(int)this_00);
  }
  ExceptionList = local_c;
  return;
}

