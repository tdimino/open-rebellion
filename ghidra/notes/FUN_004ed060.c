
void __thiscall FUN_004ed060(int param_1,undefined4 param_2,int param_3)

{
  ushort uVar1;
  undefined4 *puVar2;
  undefined2 uStack_2c;
  undefined2 uStack_2a;
  undefined2 uStack_28;
  undefined2 uStack_26;
  undefined2 uStack_24;
  undefined2 uStack_22;
  undefined2 uStack_20;
  undefined2 uStack_1e;
  undefined2 uStack_1c;
  undefined2 uStack_1a;
  undefined1 auStack_18 [12];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0063ecc8;
  pvStack_c = ExceptionList;
  if (param_3 != 7) {
    ExceptionList = &pvStack_c;
    FUN_004f6030(param_2,param_3);
    ExceptionList = pvStack_c;
    return;
  }
  ExceptionList = &pvStack_c;
  FUN_005f2f50();
  uVar1 = *(ushort *)(param_1 + 0x5c);
  uStack_4 = 0;
  if (uVar1 < 10) {
    if (0x13 < uVar1) goto LAB_004ed0e1;
    if (0x4f < uVar1) goto LAB_004ed104;
    if (99 < uVar1) goto LAB_004ed126;
    if (uVar1 < 0x78) {
      uStack_1c = 0x8754;
      uStack_1a = DAT_0065d424;
      puVar2 = (undefined4 *)&uStack_1c;
      goto LAB_004ed179;
    }
  }
  else {
    if (uVar1 < 0x14) {
      puVar2 = &param_3;
      param_3 = CONCAT22(DAT_0065d424,0x8749);
      goto LAB_004ed179;
    }
LAB_004ed0e1:
    if (uVar1 < 0x50) {
      puVar2 = (undefined4 *)&uStack_2c;
      uStack_2c = 0x8750;
      uStack_2a = DAT_0065d424;
      goto LAB_004ed179;
    }
LAB_004ed104:
    if (uVar1 < 100) {
      uStack_28 = 0x8751;
      uStack_26 = DAT_0065d424;
      puVar2 = (undefined4 *)&uStack_28;
      goto LAB_004ed179;
    }
LAB_004ed126:
    if (uVar1 < 0x78) {
      puVar2 = (undefined4 *)&uStack_24;
      uStack_24 = 0x8752;
      uStack_22 = DAT_0065d424;
      goto LAB_004ed179;
    }
  }
  puVar2 = (undefined4 *)&uStack_20;
  uStack_20 = 0x8753;
  uStack_1e = DAT_0065d424;
LAB_004ed179:
  FUN_005f3010(puVar2);
  FUN_005f30d0(auStack_18);
  uStack_4 = 0xffffffff;
  FUN_005f2ff0();
  ExceptionList = pvStack_c;
  return;
}

