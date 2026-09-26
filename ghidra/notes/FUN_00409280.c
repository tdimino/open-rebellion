
void FUN_00409280(void)

{
  undefined4 *puVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00629ba5;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005975a0();
  if (DAT_006b1214 != (undefined4 *)0x0) {
    (**(code **)*DAT_006b1214)(1);
    DAT_006b1214 = (undefined4 *)0x0;
  }
  puVar1 = (undefined4 *)FUN_00618b70(0x14);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  uStack_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1[1] = 1;
    puVar1[2] = 1;
    FUN_0042dbe0(puVar1 + 3);
    uStack_4._0_1_ = 1;
    FUN_0042dbe0(puVar1 + 4);
    *puVar1 = &PTR_FUN_00658644;
    uStack_4 = CONCAT31(uStack_4._1_3_,2);
    DAT_006b2ae0 = &LAB_0041d100;
    DAT_006b2ae8 = &LAB_0041dc70;
    FUN_004fcde0();
  }
  uStack_4 = 0xffffffff;
  DAT_006b1214 = puVar1;
  if ((puVar1 != (undefined4 *)0x0) && (puVar1[1] == 0)) {
    FUN_005975a0();
    if (DAT_006b1214 != (undefined4 *)0x0) {
      (**(code **)*DAT_006b1214)(1);
      DAT_006b1214 = (undefined4 *)0x0;
    }
  }
  ExceptionList = pvStack_c;
  return;
}

