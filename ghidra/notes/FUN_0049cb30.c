
undefined4 __fastcall FUN_0049cb30(int *param_1)

{
  void *this;
  undefined4 *puVar1;
  int iVar2;
  undefined1 local_14 [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006370a8;
  local_c = ExceptionList;
  switch(param_1[0x1e]) {
  case 1:
    ExceptionList = &local_c;
    FUN_0042dd00(param_1 + 0xd,*param_1);
    this = (void *)FUN_004f3dd0(*param_1,*param_1);
    if (this != (void *)0x0) {
      puVar1 = FUN_0041bde0(this,local_14);
      local_4 = 0;
      FUN_0049cd70(param_1,puVar1);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_0049cd90(param_1,*(undefined4 *)((int)this + 0x70));
    }
    param_1[0x1e] = 2;
    ExceptionList = local_c;
    return 0;
  case 2:
    ExceptionList = &local_c;
    iVar2 = FUN_004c5c00(param_1 + 2);
    if (iVar2 != 0) {
      param_1[0x1e] = 3;
      ExceptionList = local_c;
      return 0;
    }
    break;
  case 3:
    ExceptionList = &local_c;
    iVar2 = FUN_004c5740(param_1 + 8,param_1 + 2,param_1);
    if (iVar2 != 0) {
      param_1[0x1e] = 4;
      ExceptionList = local_c;
      return 0;
    }
    break;
  case 4:
    ExceptionList = &local_c;
    iVar2 = FUN_0049cc80(param_1);
    if (iVar2 != 0) {
      ExceptionList = local_c;
      return 1;
    }
    break;
  default:
    param_1[0x1e] = 1;
  }
  ExceptionList = local_c;
  return 0;
}

