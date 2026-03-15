
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl FUN_005b1f60(int param_1)

{
  uint *puVar1;
  void *pvVar2;
  int *piVar3;
  int *piVar4;
  int iVar5;
  uint uVar6;
  int *piVar7;
  
  puVar1 = *(uint **)(param_1 + 4);
  uVar6 = *puVar1;
  pvVar2 = (void *)FUN_00596bd0();
  piVar3 = (int *)FUN_005a6340(pvVar2,uVar6);
  uVar6 = puVar1[1];
  pvVar2 = (void *)FUN_00596bd0();
  piVar4 = (int *)FUN_005a6340(pvVar2,uVar6);
  if ((piVar3 != (int *)0x0) && (piVar4 != (int *)0x0)) {
    piVar7 = piVar3;
    FUN_005aaf90();
    iVar5 = FUN_005ad870(piVar7);
    if (iVar5 != 0) {
      piVar7 = piVar4;
      FUN_005aaf90();
      iVar5 = FUN_005ad870(piVar7);
      if (iVar5 != 0) {
        (**(code **)(*piVar4 + 200))();
        iVar5 = FUN_005aaf90();
        *(undefined4 *)(iVar5 + 0x88c) = 1;
        if (_DAT_0066c4dc <= (float)puVar1[2]) {
          iVar5 = (**(code **)(*piVar3 + 0x24))();
          if (iVar5 != 0xb) {
            iVar5 = (**(code **)(*piVar3 + 0x24))();
            if (iVar5 != 0xe) {
              iVar5 = (**(code **)(*piVar3 + 0x24))();
              if (iVar5 != 0xd) {
                iVar5 = (**(code **)(*piVar3 + 0x24))();
                if (iVar5 != 0x16) {
                  iVar5 = (**(code **)(*piVar3 + 0x24))();
                  if (iVar5 != 0x19) {
                    FUN_005c12d0(puVar1[1],*puVar1,1,0x3f800000,3);
                    return;
                  }
                }
              }
            }
          }
          FUN_005c12d0(puVar1[1],*puVar1,1,0x3f400000,2);
          return;
        }
        FUN_005c12d0(puVar1[1],*puVar1,1,0x3f266666,1);
      }
    }
  }
  return;
}

