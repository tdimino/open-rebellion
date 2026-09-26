
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __cdecl FUN_0055e010(uint *param_1,uint *param_2,undefined4 *param_3)

{
  undefined4 *puVar1;
  uint uVar2;
  undefined4 uVar3;
  uint uVar4;
  int iVar5;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649e78;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = FUN_00562ae0(local_28);
  local_4 = 0;
  FUN_00562b80(param_3,puVar1);
  local_4 = 0xffffffff;
  FUN_00562b30();
  if (((*param_1 ^ *param_2) & 3) != 0) {
    uVar4 = *param_1 & 3;
    uVar2 = *param_2 & 3;
    if (uVar4 == 3) {
      if (uVar2 == 3) {
        uVar4 = 3;
      }
      else {
        uVar4 = 2 - (uVar2 != 1);
      }
    }
    if (uVar2 == 3) {
      if (uVar4 == 3) {
        uVar2 = 3;
      }
      else {
        uVar2 = 2 - (uVar4 != 1);
      }
    }
    *param_3 = 0;
    FUN_0055de00((int)param_3);
    param_3[3] = uVar4;
    param_3[5] = uVar2;
    uVar3 = DAT_006bb684;
    if (uVar2 != 1) {
      uVar3 = DAT_006bb5d4;
    }
    if (((*param_1 & 3) == 1) || (iVar5 = DAT_00661a84, ((byte)*param_2 & 3) == 1)) {
      iVar5 = DAT_00661a84 + _DAT_006bb668;
    }
    if (((*param_1 & 3) == 2) || (((byte)*param_2 & 3) == 2)) {
      iVar5 = iVar5 + _DAT_006bb60c;
    }
    param_3[4] = uVar3;
    param_3[6] = iVar5;
  }
  ExceptionList = local_c;
  return 1;
}

