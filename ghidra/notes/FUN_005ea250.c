
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall FUN_005ea250(int *param_1,uint param_2)

{
  float fVar1;
  int iVar2;
  undefined4 *puVar3;
  int iVar4;
  uint uVar5;
  undefined4 *puVar6;
  undefined4 uVar7;
  uint uVar8;
  undefined1 auStack_1c [8];
  int iStack_14;
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00655301;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (DAT_006bc470 == (undefined4 *)0x0) {
    ExceptionList = &pvStack_c;
    puVar3 = (undefined4 *)FUN_00618b70(0x48);
    uStack_4 = 0;
    if (puVar3 == (undefined4 *)0x0) {
      DAT_006bc470 = (undefined4 *)0x0;
    }
    else {
      DAT_006bc470 = FUN_005a6290(puVar3);
    }
  }
  uStack_4 = 0xffffffff;
  iVar4 = FUN_005a6340(DAT_006bc470,param_2);
  uVar7 = 0;
  if (iVar4 != 0) {
    FUN_005c5f20(param_1,iVar4);
    uVar8 = param_1[0x4b] + 1;
    param_1[0x4b] = uVar8;
    uVar5 = param_1[0x4c];
    if ((uint)param_1[0x4c] <= uVar8) {
      uVar5 = uVar8;
    }
    param_1[0x4c] = uVar5;
    fVar1 = *(float *)(iVar4 + 0xc) * _DAT_0066d870;
    if (fVar1 < (float)param_1[0x4f]) {
      fVar1 = (float)param_1[0x4f];
    }
    param_1[0x4f] = (int)fVar1;
    if (uVar8 == 1) {
      iVar2 = *(int *)(iVar4 + 0x34);
      iStack_14 = *(int *)(iVar4 + 0x38);
      iStack_10 = *(int *)(iVar4 + 0x3c);
      param_1[0xf] = *(int *)(iVar4 + 0x30);
      param_1[0x10] = iVar2;
      param_1[0x11] = iStack_14;
      param_1[0x12] = iStack_10;
      param_1[0x40] = param_2;
      puVar3 = (undefined4 *)FUN_00618b70(0xf4);
      uStack_4 = 1;
      if (puVar3 == (undefined4 *)0x0) {
        puVar3 = (undefined4 *)0x0;
      }
      else {
        FUN_005e9cc0(puVar3,iVar4);
        uStack_4 = CONCAT31(uStack_4._1_3_,2);
        FUN_005ebc90(puVar3 + 8);
        *puVar3 = &PTR_FUN_0066d968;
        puVar3[8] = &PTR_LAB_0066d948;
      }
      uStack_4 = 0xffffffff;
      puVar6 = (undefined4 *)(**(code **)(*param_1 + 0x30))(auStack_1c,iVar4);
      puVar3[0x14] = *puVar6;
      puVar3[0x15] = puVar6[1];
      puVar3[0x16] = puVar6[2];
      puVar3[0x17] = puVar6[3];
    }
    else {
      puVar3 = (undefined4 *)FUN_00618b70(0xf4);
      uStack_4 = 3;
      if (puVar3 == (undefined4 *)0x0) {
        puVar3 = (undefined4 *)0x0;
      }
      else {
        FUN_005e9cc0(puVar3,iVar4);
        uStack_4 = CONCAT31(uStack_4._1_3_,4);
        FUN_0059faf0(puVar3 + 0xc);
        FUN_0059faf0(puVar3 + 0x10);
        FUN_0059faf0(puVar3 + 0x14);
        FUN_0059faf0(puVar3 + 0x18);
        FUN_0059faf0(puVar3 + 0x1f);
        FUN_0059faf0(puVar3 + 0x23);
        FUN_0059faf0(puVar3 + 0x27);
        FUN_0059faf0(puVar3 + 0x2b);
        FUN_0059faf0(puVar3 + 0x2f);
        FUN_0059faf0(puVar3 + 0x33);
        FUN_0059faf0(puVar3 + 0x37);
        puVar3[8] = &PTR_FUN_0066c648;
        puVar3[9] = 0x3f800000;
        puVar3[0x1c] = 0;
        FUN_0059fb10(puVar3 + 0xc,0,0,0x3f800000);
        *puVar3 = &PTR_FUN_0066d968;
        puVar3[8] = &PTR_LAB_0066d948;
      }
      uStack_4 = 0xffffffff;
      puVar6 = (undefined4 *)(**(code **)(*param_1 + 0x30))(auStack_1c,iVar4);
      puVar3[0x14] = *puVar6;
      puVar3[0x15] = puVar6[1];
      puVar3[0x16] = puVar6[2];
      puVar3[0x17] = puVar6[3];
    }
    FUN_005c61a0(param_1 + 0x38,puVar3);
    uVar7 = 1;
  }
  ExceptionList = pvStack_c;
  return uVar7;
}

