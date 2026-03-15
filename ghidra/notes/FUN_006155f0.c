
void __fastcall FUN_006155f0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006570b8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066e4e4;
  local_4 = 0;
  if ((param_1[0x13] != 0) && ((undefined *)param_1[4] != (undefined *)0x0)) {
    if ((code *)param_1[0x18] == (code *)0x0) {
      FUN_00618b60((undefined *)param_1[4]);
    }
    else {
      (*(code *)param_1[0x18])();
    }
  }
  local_4 = 0xffffffff;
  FUN_00617dd0(param_1);
  ExceptionList = pvStack_c;
  return;
}

