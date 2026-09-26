
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __fastcall FUN_0059a530(void *param_1)

{
  float10 fVar1;
  void *this;
  int iVar2;
  int *piVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  float10 fVar7;
  float fVar8;
  
  if (*(int *)((int)param_1 + 0x690) == 0) {
    if (*(int *)((int)param_1 + 0x668) == 1) {
      fVar8 = DAT_0066badc;
      this = (void *)FUN_00596f30();
      iVar2 = FUN_005ae9c0(this,fVar8);
      if (iVar2 != 0) {
        iVar2 = FUN_00596f30();
        puVar5 = (undefined4 *)(iVar2 + 0x10);
        puVar6 = (undefined4 *)((int)param_1 + 0x66c);
        for (iVar4 = 7; iVar4 != 0; iVar4 = iVar4 + -1) {
          *puVar6 = *puVar5;
          puVar5 = puVar5 + 1;
          puVar6 = puVar6 + 1;
        }
        return 1;
      }
    }
    else {
      iVar2 = FUN_00401300();
      if (iVar2 == 1) {
        iVar2 = FUN_005aaf90();
        if (*(int *)(iVar2 + 0x8c8) != 0) {
          FUN_004012f0();
          return 0;
        }
        piVar3 = (int *)FUN_005aaf90();
        (**(code **)(*piVar3 + 0xc))();
        FUN_004012f0();
        piVar3 = (int *)FUN_005aaf90();
        (**(code **)(*piVar3 + 8))();
        return 0;
      }
      iVar2 = FUN_005aaf90();
      if (*(int *)(iVar2 + 0x8c4) != 0) {
LAB_0059a62d:
        iVar2 = *(int *)((int)param_1 + 0x690) + 1;
        *(int *)((int)param_1 + 0x690) = iVar2;
        if (iVar2 < 0) {
          *(undefined4 *)((int)param_1 + 0x690) = 0;
        }
        iVar2 = FUN_005aaf90();
        puVar5 = (undefined4 *)(iVar2 + 0x9f0);
        puVar6 = (undefined4 *)((int)param_1 + 0x66c);
        for (iVar4 = 7; iVar4 != 0; iVar4 = iVar4 + -1) {
          *puVar6 = *puVar5;
          puVar5 = puVar5 + 1;
          puVar6 = puVar6 + 1;
        }
        FUN_00599bc0((int)param_1);
        return 1;
      }
      fVar7 = FUN_005fbbe0((int)param_1 + 0x698);
      fVar1 = (float10)_DAT_0066bae0;
      FUN_005fbbb0((int)param_1 + 0x698);
      if (*(int *)((int)param_1 + 0x6b0) == 0) {
        FUN_0059a6a0(param_1,(float)(fVar7 * fVar1));
        iVar2 = FUN_005aaf90();
        if (*(int *)(iVar2 + 0x8c4) != 0) goto LAB_0059a62d;
      }
      else {
        *(undefined4 *)((int)param_1 + 0x6b0) = 0;
      }
      iVar2 = FUN_005c1490();
      while (iVar2 == 0) {
        iVar2 = FUN_005c1490();
      }
      FUN_005c1470((float)fVar7);
    }
  }
  return 0;
}

