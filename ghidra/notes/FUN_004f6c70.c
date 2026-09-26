
int __fastcall FUN_004f6c70(int *param_1)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  uint uVar4;
  int iVar5;
  int local_3c;
  undefined4 auStack_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0063fd20;
  local_c = ExceptionList;
  iVar5 = 1;
  local_3c = 0;
  ExceptionList = &local_c;
  do {
    if (iVar5 == 0) {
      ExceptionList = local_c;
      return 0;
    }
    piVar2 = (int *)FUN_00539fd0(param_1,local_3c);
    if ((*(byte *)(piVar2 + 0x14) & 8) == 0) {
      iVar5 = 0;
    }
    if (iVar5 != 0) {
      iVar3 = (**(code **)(*piVar2 + 0x44))();
      if (iVar3 == 0) {
        iVar5 = 0;
      }
      if (iVar5 != 0) {
        FUN_004fcd00(auStack_2c,piVar2,4);
        uStack_4 = 0;
        bVar1 = FUN_005131b0((int)auStack_2c);
        if (CONCAT31(extraout_var,bVar1) != 0) {
          iVar5 = 0;
        }
        uStack_4 = 0xffffffff;
        FUN_004fcd80(auStack_2c);
        if ((iVar5 != 0) && (bVar1 = FUN_005406d0(param_1), CONCAT31(extraout_var_00,bVar1) != 0)) {
          if ((int *)param_1[7] == (int *)0x0) {
            iVar5 = 0;
          }
          else {
            uStack_4 = 1;
            uVar4 = (**(code **)(*(int *)param_1[7] + 4))();
            if ((uVar4 < 0xf2) || (0xf2 < uVar4)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            uStack_4 = 0xffffffff;
            FUN_00619730();
            if (!bVar1) {
              iVar5 = 0;
            }
          }
        }
      }
    }
    local_3c = local_3c + 1;
  } while (local_3c < 3);
  ExceptionList = local_c;
  return iVar5;
}

