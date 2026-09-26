
uint __thiscall FUN_0052ac40(void *this,void *param_1)

{
  uint *puVar1;
  undefined4 *puVar2;
  void *this_00;
  uint uVar3;
  int iVar4;
  uint local_14;
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644a18;
  local_c = ExceptionList;
  iVar4 = 1;
  ExceptionList = &local_c;
  FUN_004ece30(&local_14);
  local_4 = 0;
  puVar1 = FUN_0040d760(this,local_10);
  local_4._0_1_ = 1;
  puVar1 = FUN_004ece40(puVar1);
  local_4._0_1_ = 0;
  FUN_00619730();
  if (puVar1 == (uint *)0x0) {
    puVar2 = FUN_004c27b0(this,local_10);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_14,puVar2);
    local_4._0_1_ = 0;
    FUN_00619730();
  }
  else {
    this_00 = (void *)thunk_FUN_0052be30(this);
    uVar3 = 0;
    if (this_00 == (void *)0x0) goto LAB_0052ad08;
    iVar4 = FUN_004f6b70(this_00,&local_14);
  }
  uVar3 = 0;
  if (iVar4 != 0) {
    uVar3 = FUN_00529820(this,(int *)&local_14,param_1);
  }
LAB_0052ad08:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar3;
}

