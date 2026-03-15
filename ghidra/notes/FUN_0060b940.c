
void __cdecl FUN_0060b940(uint param_1,uint param_2,HMODULE param_3)

{
  undefined *puVar1;
  void *this;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar1 = DAT_006be5c4;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006569bb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (DAT_006be5c4 != (undefined *)0x0) {
    ExceptionList = &local_c;
    FUN_00619730();
    FUN_00618b60(puVar1);
    DAT_006be5c4 = (undefined *)0x0;
  }
  if (param_3 != (HMODULE)0x0) {
    this = (void *)FUN_00618b70(0xc);
    local_4 = 0;
    if (this == (void *)0x0) {
      DAT_006be5c4 = (undefined *)0x0;
    }
    else {
      DAT_006be5c4 = (undefined *)FUN_0060b7b0(this,param_1,param_2,param_3);
    }
  }
  ExceptionList = local_c;
  return;
}

