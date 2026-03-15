
void __cdecl FUN_0061d0a0(int param_1,int param_2,byte *param_3,byte *param_4)

{
  int *piVar1;
  bool bVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar3;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  undefined4 *puVar4;
  undefined3 extraout_var_05;
  undefined3 extraout_var_06;
  undefined3 extraout_var_07;
  uint uVar5;
  void *local_14;
  code *pcStack_10;
  undefined *puStack_c;
  undefined4 local_8;
  
  puStack_c = &DAT_0066e6f0;
  pcStack_10 = FUN_0061a414;
  local_14 = ExceptionList;
  if (((*(int *)(param_3 + 4) != 0) && (*(char *)(*(int *)(param_3 + 4) + 8) != '\0')) &&
     (*(int *)(param_3 + 8) != 0)) {
    piVar1 = (int *)(param_2 + 0xc + *(int *)(param_3 + 8));
    local_8 = 0;
    if ((*param_3 & 8) == 0) {
      if ((*param_4 & 1) == 0) {
        if (*(int *)(param_4 + 0x18) == 0) {
          ExceptionList = &local_14;
          bVar2 = FUN_00624a00(*(void **)(param_1 + 0x18),1);
          if ((CONCAT31(extraout_var_03,bVar2) != 0) &&
             (bVar2 = FUN_00624a20(piVar1,1), CONCAT31(extraout_var_04,bVar2) != 0)) {
            uVar5 = *(uint *)(param_4 + 0x14);
            puVar4 = (undefined4 *)FUN_0061d330(*(int *)(param_1 + 0x18),(int *)(param_4 + 8));
            FUN_0061be90(piVar1,puVar4,uVar5);
            ExceptionList = local_14;
            return;
          }
        }
        else {
          ExceptionList = &local_14;
          bVar2 = FUN_00624a00(*(void **)(param_1 + 0x18),1);
          if (((CONCAT31(extraout_var_05,bVar2) != 0) &&
              (bVar2 = FUN_00624a20(piVar1,1), CONCAT31(extraout_var_06,bVar2) != 0)) &&
             (bVar2 = FUN_00624a40(*(FARPROC *)(param_4 + 0x18)),
             CONCAT31(extraout_var_07,bVar2) != 0)) {
            if ((*param_4 & 4) != 0) {
              FUN_0061d330(*(int *)(param_1 + 0x18),(int *)(param_4 + 8));
              FUN_00618750(piVar1,*(undefined **)(param_4 + 0x18));
              ExceptionList = local_14;
              return;
            }
            FUN_0061d330(*(int *)(param_1 + 0x18),(int *)(param_4 + 8));
            FUN_00618750(piVar1,*(undefined **)(param_4 + 0x18));
            ExceptionList = local_14;
            return;
          }
        }
      }
      else {
        ExceptionList = &local_14;
        bVar2 = FUN_00624a00(*(void **)(param_1 + 0x18),1);
        if ((CONCAT31(extraout_var_01,bVar2) != 0) &&
           (bVar2 = FUN_00624a20(piVar1,1), CONCAT31(extraout_var_02,bVar2) != 0)) {
          FUN_0061be90(piVar1,*(undefined4 **)(param_1 + 0x18),*(uint *)(param_4 + 0x14));
          if (*(int *)(param_4 + 0x14) != 4) {
            ExceptionList = local_14;
            return;
          }
          if (*piVar1 == 0) {
            ExceptionList = local_14;
            return;
          }
          iVar3 = FUN_0061d330(*piVar1,(int *)(param_4 + 8));
          *piVar1 = iVar3;
          ExceptionList = local_14;
          return;
        }
      }
    }
    else {
      ExceptionList = &local_14;
      bVar2 = FUN_00624a00(*(void **)(param_1 + 0x18),1);
      if ((CONCAT31(extraout_var,bVar2) != 0) &&
         (bVar2 = FUN_00624a20(piVar1,1), CONCAT31(extraout_var_00,bVar2) != 0)) {
        iVar3 = *(int *)(param_1 + 0x18);
        *piVar1 = iVar3;
        iVar3 = FUN_0061d330(iVar3,(int *)(param_4 + 8));
        *piVar1 = iVar3;
        ExceptionList = local_14;
        return;
      }
    }
    FUN_0061d540();
  }
  ExceptionList = local_14;
  return;
}

