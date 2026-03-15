
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005c81d0(int *param_1)

{
  float10 fVar1;
  int iVar2;
  int *piVar3;
  int *piVar4;
  undefined4 *puVar5;
  undefined4 uVar6;
  float10 fVar7;
  float10 fVar8;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00653e9b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar2 = (**(code **)(*param_1 + 0x14))(0);
  param_1[0xe] = param_1[0xf];
  param_1[0x11] = iVar2;
  for (piVar3 = (int *)(**(code **)(param_1[0x16] + 0x10))(); piVar3 != (int *)0x0;
      piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
    piVar4 = (int *)(**(code **)(*piVar3 + 0x10))();
    fVar7 = (float10)(**(code **)(*piVar4 + 0x90))();
    piVar4 = (int *)(**(code **)(*piVar3 + 0x10))();
    fVar8 = FUN_005b18d0(piVar4);
    fVar1 = (float10)_DAT_0066cbb0;
    puVar5 = (undefined4 *)FUN_00618b70(0x20);
    puStack_8 = (undefined1 *)0x0;
    if (puVar5 == (undefined4 *)0x0) {
      puVar5 = (undefined4 *)0x0;
    }
    else {
      uVar6 = (**(code **)(*piVar3 + 0x10))();
      FUN_005f5c10(puVar5);
      puVar5[6] = -(float)((float10)(float)fVar7 - fVar8 * fVar1);
      puVar5[7] = uVar6;
      *puVar5 = &PTR_FUN_0066cc6c;
    }
    puStack_8 = (undefined1 *)0xffffffff;
    iVar2 = (**(code **)(*piVar3 + 0x10))();
    iVar2 = FUN_005b7d10(iVar2);
    if (iVar2 == 1) {
      piVar4 = param_1 + 0x2a;
    }
    else {
      iVar2 = (**(code **)(*piVar3 + 0x10))();
      iVar2 = FUN_005b7ce0(iVar2);
      piVar4 = param_1 + 0x28;
      if (iVar2 == 0) {
        piVar4 = param_1 + 0x26;
      }
    }
    FUN_005f4f10(piVar4,(int)puVar5);
  }
  ExceptionList = pvStack_10;
  return;
}

