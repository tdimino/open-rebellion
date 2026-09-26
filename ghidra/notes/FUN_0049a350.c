
void __fastcall FUN_0049a350(void *param_1)

{
  bool bVar1;
  undefined4 uVar2;
  int iVar3;
  void *pvVar4;
  uint *puVar5;
  int *piVar6;
  uint uVar7;
  int *piVar8;
  int iStack_34;
  undefined1 auStack_30 [8];
  undefined1 auStack_28 [4];
  undefined1 auStack_24 [4];
  undefined1 auStack_20 [4];
  uint uStack_1c;
  uint uStack_18;
  void *pvStack_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00636ae8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f35e0((void *)((int)param_1 + 0x44),&DAT_006b120c);
  uVar2 = (**(code **)(**(int **)((int)param_1 + 0x5c) + 0xc))();
  switch(uVar2) {
  case 0x200:
    *(ushort *)((int)param_1 + 0x30) = (*(int *)((int)param_1 + 0x20) != 1) + 0x461;
    FUN_0060b9d0((void *)((int)param_1 + 0x50),0x7050,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0,(undefined4 *)0x0);
    FUN_004f4340(&iStack_34);
    uStack_4 = 0;
    (**(code **)(**(int **)((int)param_1 + 0x5c) + 0x3c))(&iStack_34);
    for (pvVar4 = (void *)thunk_FUN_005f5060((int)&stack0xffffffc8); pvVar4 != (void *)0x0;
        pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
      puVar5 = FUN_00403040(pvVar4,auStack_30);
      puStack_8._0_1_ = 1;
      piVar6 = FUN_004f2d10(*(int *)((int)param_1 + 0x20),puVar5);
      puStack_8 = (undefined1 *)((uint)puStack_8._1_3_ << 8);
      FUN_00619730();
      if (piVar6 != (int *)0x0) {
        FUN_0049a880(param_1,(int)piVar6);
      }
    }
    puStack_8 = (undefined1 *)0xffffffff;
    *(ushort *)((int)param_1 + 0x2e) = (*(int *)((int)param_1 + 0x20) != 1) + 0x408;
    FUN_004f4380((undefined4 *)&stack0xffffffc8);
    ExceptionList = pvStack_10;
    return;
  case 0x201:
  case 0x202:
    break;
  default:
    FUN_005f35e0((void *)((int)param_1 + 0x44),&DAT_006b120c);
    FUN_005f35e0((void *)((int)param_1 + 0x50),&DAT_006b120c);
    ExceptionList = pvStack_c;
    return;
  case 0x213:
    *(undefined2 *)((int)param_1 + 0x30) = 0x465;
    FUN_0060b9d0((void *)((int)param_1 + 0x50),0x7053,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0,(undefined4 *)0x0);
    iVar3 = (**(code **)(**(int **)((int)param_1 + 0x5c) + 0x28))();
    for (pvVar4 = (void *)thunk_FUN_005f5060(iVar3); pvVar4 != (void *)0x0;
        pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
      puVar5 = FUN_00403040(pvVar4,auStack_28);
      uStack_4 = 2;
      piVar6 = FUN_004f2f60(*(int *)((int)param_1 + 0x20),puVar5);
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (piVar6 == (int *)0x0) {
        piVar6 = (int *)0x0;
      }
      else {
        piVar6 = piVar6 + 0xc;
      }
      FUN_0060b9d0((void *)((int)param_1 + 0x50),0x7055,piVar6,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
    }
    *(ushort *)((int)param_1 + 0x2e) = (*(int *)((int)param_1 + 0x20) != 1) + 0x40e;
    ExceptionList = pvStack_c;
    return;
  case 0x242:
    *(ushort *)((int)param_1 + 0x30) = (*(int *)((int)param_1 + 0x20) != 1) + 0x461;
    FUN_0060b9d0((void *)((int)param_1 + 0x50),0x7051,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0,(undefined4 *)0x0);
    iVar3 = (**(code **)(**(int **)((int)param_1 + 0x5c) + 0x28))();
    for (pvVar4 = (void *)thunk_FUN_005f5060(iVar3); pvVar4 != (void *)0x0;
        pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
      puVar5 = FUN_00403040(pvVar4,auStack_24);
      uStack_4 = 3;
      piVar6 = FUN_004f2d10(*(int *)((int)param_1 + 0x20),puVar5);
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (piVar6 != (int *)0x0) {
        FUN_0060b9d0((void *)((int)param_1 + 0x50),0x7054,piVar6 + 0xc,(undefined4 *)0x0,
                     (undefined4 *)0x0,(undefined4 *)0x0);
      }
    }
    *(ushort *)((int)param_1 + 0x2e) = (*(int *)((int)param_1 + 0x20) != 1) + 0x40a;
    ExceptionList = pvStack_c;
    return;
  case 0x250:
    FUN_0060b9d0((void *)((int)param_1 + 0x50),0x7052,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0,(undefined4 *)0x0);
    *(ushort *)((int)param_1 + 0x30) = (*(int *)((int)param_1 + 0x20) != 1) + 0x463;
    *(ushort *)((int)param_1 + 0x2e) = (*(int *)((int)param_1 + 0x20) != 1) + 0x40c;
    ExceptionList = pvStack_c;
    return;
  }
  piVar6 = (int *)0x0;
  iStack_34 = 0;
  piVar8 = (int *)0x0;
  iVar3 = (**(code **)(**(int **)((int)param_1 + 0x5c) + 0x28))();
  pvVar4 = (void *)thunk_FUN_005f5060(iVar3);
  if (pvVar4 != (void *)0x0) {
    puVar5 = FUN_00403040(pvVar4,auStack_20);
    uStack_4 = 4;
    piVar8 = FUN_004f2d10(*(int *)((int)param_1 + 0x20),puVar5);
    uStack_4 = 0xffffffff;
    FUN_00619730();
  }
  if (piVar8 != (int *)0x0) {
    uStack_1c = 8;
    uStack_18 = 0x10;
    uStack_4 = 5;
    uVar7 = (**(code **)(*piVar8 + 4))();
    if ((uVar7 < uStack_1c) || (uStack_18 <= uVar7)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    uStack_4 = 0xffffffff;
    FUN_00619730();
    if (!bVar1) {
      piVar8 = (int *)piVar8[7];
      pvStack_14 = (void *)0x90;
      pvStack_10 = (void *)0x98;
      uStack_4 = 6;
      pvVar4 = (void *)(**(code **)(*piVar8 + 4))();
      if ((pvVar4 < pvStack_14) || (pvStack_10 <= pvVar4)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (bVar1) {
        piVar6 = piVar8;
      }
    }
  }
  if ((piVar6 != (int *)0x0) && ((*(byte *)(piVar6 + 0x22) & 0x20) != 0)) {
    iStack_34 = 1;
  }
  if (iStack_34 == 0) {
    FUN_0060b9d0((void *)((int)param_1 + 0x50),0x7057,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0,(undefined4 *)0x0);
    *(ushort *)((int)param_1 + 0x2e) = (*(int *)((int)param_1 + 0x20) != 1) + 0x3fa;
    FUN_0049a8b0((int)param_1);
    ExceptionList = pvStack_c;
    return;
  }
  FUN_0060b9d0((void *)((int)param_1 + 0x50),0x7056,(undefined4 *)0x0,(undefined4 *)0x0,
               (undefined4 *)0x0,(undefined4 *)0x0);
  *(ushort *)((int)param_1 + 0x2e) = 0x407 - (ushort)(*(int *)((int)param_1 + 0x20) != 1);
  FUN_0049a8b0((int)param_1);
  ExceptionList = pvStack_c;
  return;
}

