
void __thiscall FUN_00512d00(void *this,undefined4 param_1,int param_2,void *param_3)

{
  int *this_00;
  int iVar1;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00642248;
  local_c = ExceptionList;
  local_10 = 0x8000004;
  local_4 = 0;
  ExceptionList = &local_c;
  this_00 = FUN_004f7d50(this,&local_10,param_1,param_3,0,0);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (this_00 != (int *)0x0) {
    iVar1 = FUN_004fe230(this_00,0,param_3);
    if (iVar1 != 0) {
      if (param_2 != 0) {
        FUN_004fe2a0(this_00,1,param_3);
        ExceptionList = local_c;
        return;
      }
      FUN_004fe310(this_00,1,param_3);
    }
  }
  ExceptionList = local_c;
  return;
}

