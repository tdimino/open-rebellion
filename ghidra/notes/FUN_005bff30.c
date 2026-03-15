
void __thiscall FUN_005bff30(void *this,uint param_1)

{
  DWORD DVar1;
  uint *puVar2;
  void *this_00;
  undefined4 *this_01;
  int iVar3;
  DWORD DVar4;
  undefined4 auStack_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00653873;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  DVar1 = timeGetTime();
  puVar2 = FUN_00597110('\x03');
  do {
    while (puVar2 != (uint *)0x0) {
      this_00 = (void *)FUN_00618b70(0x220);
      uStack_4 = 0;
      if (this_00 == (void *)0x0) {
        this_01 = (undefined4 *)0x0;
      }
      else {
        this_01 = (undefined4 *)FUN_005ce3e0(this_00,puVar2);
      }
      uStack_4 = 0xffffffff;
      FUN_005f5fa0(auStack_14);
      uStack_4 = 1;
      FUN_005ce340(this_01,auStack_14);
                    /* WARNING: Load size is inaccurate */
      iVar3 = (**(code **)(*this + 0x74))(this_01);
      if (iVar3 == 0) {
                    /* WARNING: Load size is inaccurate */
        (**(code **)(*this + 0x2c))(this_01,0);
      }
      else if (this_01 != (undefined4 *)0x0) {
        (**(code **)*this_01)(1);
      }
      if ((undefined *)puVar2[1] != (undefined *)0x0) {
        FUN_00618b60((undefined *)puVar2[1]);
      }
      puVar2[1] = 0;
      FUN_00618b60((undefined *)puVar2);
      puVar2 = FUN_00597110('\x03');
      uStack_4 = 0xffffffff;
      FUN_005f5fe0(auStack_14);
    }
    DVar4 = timeGetTime();
  } while (DVar4 - DVar1 <= param_1);
  FUN_00597140();
  ExceptionList = pvStack_c;
  return;
}

