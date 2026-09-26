
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __cdecl FUN_0055de50(uint *param_1,uint *param_2,undefined4 *param_3)

{
  byte bVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649e58;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar2 = FUN_00562ae0(local_28);
  local_4 = 0;
  FUN_00562b80(param_3,puVar2);
  local_4 = 0xffffffff;
  FUN_00562b30();
  if (((*param_1 ^ *param_2) & 3) != 0) {
    uVar5 = *param_1 & 3;
    uVar4 = *param_2 & 3;
    if (uVar5 == 3) {
      if (uVar4 == 3) {
        uVar5 = 3;
      }
      else {
        uVar5 = 2 - (uVar4 != 1);
      }
    }
    if (uVar4 == 3) {
      if (uVar5 == 3) {
        uVar4 = 3;
      }
      else {
        uVar4 = 2 - (uVar5 != 1);
      }
    }
    *param_3 = 0;
    FUN_0055de00((int)param_3);
    param_3[3] = uVar5;
    param_3[5] = uVar4;
    if ((*param_1 & 0x30) == 0) {
      if ((*param_2 & 0x30) == 0) {
        uVar3 = DAT_006bb6e4;
        if (uVar4 != 1) {
          uVar3 = DAT_006bb608;
        }
        uVar4 = *param_1 & 3;
        bVar1 = (byte)*param_2;
        if ((uVar4 == 1) || (iVar6 = DAT_00661a84, (bVar1 & 3) == 1)) {
          iVar6 = DAT_00661a84 + _DAT_006bb660;
        }
        if ((uVar4 == 2) || ((bVar1 & 3) == 2)) {
          iVar6 = iVar6 + _DAT_006bb67c;
        }
      }
      else {
        uVar3 = DAT_006bb680;
        iVar6 = DAT_006bb6f8;
        if (uVar4 == 1) {
          uVar3 = DAT_006bb620;
          iVar6 = DAT_006bb698;
        }
      }
    }
    else if ((*param_2 & 0x30) == 0) {
      uVar3 = DAT_006bb61c;
      iVar6 = DAT_006bb68c;
      if (uVar4 == 1) {
        uVar3 = DAT_006bb64c;
        iVar6 = DAT_006bb5d8;
      }
    }
    else {
      uVar3 = DAT_006bb688;
      iVar6 = DAT_006bb5e4;
      if (uVar4 != 1) {
        uVar3 = DAT_006bb650;
        iVar6 = DAT_006bb65c;
      }
    }
    param_3[4] = uVar3;
    param_3[6] = iVar6;
  }
  ExceptionList = local_c;
  return 1;
}

