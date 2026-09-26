
void __fastcall FUN_005d6090(undefined4 *param_1)

{
  undefined4 *this;
  undefined4 *puVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006548d8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066c9b8;
  this = param_1 + 1;
  local_4 = 0;
  puVar1 = (undefined4 *)FUN_005f5060((int)this);
  while (puVar1 != (undefined4 *)0x0) {
    FUN_005f4fa0(this,(int)puVar1);
    FUN_00618b60(*(undefined **)(puVar1[9] + 0x1c));
    FUN_00618b60((undefined *)puVar1[9]);
    if (puVar1 != (undefined4 *)0x0) {
      (**(code **)*puVar1)(1);
    }
    puVar1 = (undefined4 *)FUN_005f5060((int)this);
  }
  *param_1 = &PTR_FUN_0066d2d0;
  local_4 = 0xffffffff;
  *this = &PTR_FUN_0066c9c0;
  FUN_005f4f00(this);
  ExceptionList = pvStack_c;
  return;
}

