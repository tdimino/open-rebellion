
void __fastcall FUN_004109e0(int param_1)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar4;
  int iVar5;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  int *piVar6;
  
  if (((*(int *)(param_1 + 0x164) != 0) && (*(int *)(param_1 + 0x144) != 0)) &&
     (iVar3 = FUN_00597090(), *(int *)(param_1 + 0x11c) != 0)) {
    if (iVar3 == 2) {
      bVar1 = FUN_00603120(*(void **)(param_1 + 0x164),2);
      bVar2 = FUN_00603120(*(void **)(param_1 + 0x144),2);
      if ((CONCAT31(extraout_var,bVar1) != 0) || (CONCAT31(extraout_var_00,bVar2) != 0)) {
        bVar1 = false;
        iVar3 = 1;
        piVar6 = &DAT_006b1224;
        do {
          iVar4 = FUN_00401910(iVar3,1);
          if (iVar4 == 2) {
            bVar2 = false;
            iVar5 = FUN_00411450(iVar3);
            iVar4 = 0;
            if ((int)piVar6 < 0x6b1239) {
              iVar4 = *piVar6;
            }
            if ((iVar5 != 0) && (iVar4 != 0)) {
              bVar2 = iVar5 == iVar4;
            }
            if (bVar2) {
              bVar1 = true;
              goto LAB_00410aa9;
            }
          }
          piVar6 = piVar6 + 1;
          iVar3 = iVar3 + 1;
          if (0x6b1238 < (int)piVar6) {
LAB_00410aa9:
            if (bVar1) {
              (**(code **)(**(int **)(param_1 + 0x164) + 0xc))();
            }
            (**(code **)(**(int **)(param_1 + 0x144) + 0xc))();
            return;
          }
        } while( true );
      }
    }
    else {
      bVar1 = FUN_00603120(*(void **)(param_1 + 0x164),2);
      bVar2 = FUN_00603120(*(void **)(param_1 + 0x144),2);
      if ((CONCAT31(extraout_var_01,bVar1) == 0) || (CONCAT31(extraout_var_02,bVar2) == 0)) {
        (**(code **)(**(int **)(param_1 + 0x164) + 0x10))();
        (**(code **)(**(int **)(param_1 + 0x144) + 0x10))();
      }
    }
  }
  return;
}

