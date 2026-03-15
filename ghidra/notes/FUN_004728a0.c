
void __fastcall FUN_004728a0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00632863;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065a358;
  local_4 = 1;
  if ((undefined *)param_1[8] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[8]);
    param_1[8] = 0;
  }
  if ((undefined4 *)param_1[7] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[7])(1);
    param_1[7] = 0;
  }
  local_4 = local_4 & 0xffffff00;
  FUN_004ad7b0(param_1 + 0xc);
  local_4 = 0xffffffff;
  FUN_005f55f0(param_1);
  ExceptionList = pvStack_c;
  return;
}

