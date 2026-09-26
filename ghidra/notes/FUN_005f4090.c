
undefined4 __cdecl FUN_005f4090(byte *param_1)

{
  byte bVar1;
  byte *pbVar2;
  int iVar3;
  undefined4 uVar4;
  char *pcVar5;
  bool bVar6;
  
  uVar4 = 0;
  pcVar5 = s_HKEY_LOCAL_MACHINE_006a844c;
  pbVar2 = param_1;
  do {
    bVar1 = *pbVar2;
    bVar6 = bVar1 < (byte)*pcVar5;
    if (bVar1 != *pcVar5) {
LAB_005f40c9:
      iVar3 = (1 - (uint)bVar6) - (uint)(bVar6 != 0);
      goto LAB_005f40ce;
    }
    if (bVar1 == 0) break;
    bVar1 = pbVar2[1];
    bVar6 = bVar1 < (byte)pcVar5[1];
    if (bVar1 != pcVar5[1]) goto LAB_005f40c9;
    pbVar2 = pbVar2 + 2;
    pcVar5 = pcVar5 + 2;
  } while (bVar1 != 0);
  iVar3 = 0;
LAB_005f40ce:
  if (iVar3 == 0) {
    return 0x80000002;
  }
  pcVar5 = s_HKEY_CURRENT_USER_006ac22c;
  pbVar2 = param_1;
  do {
    bVar1 = *pbVar2;
    bVar6 = bVar1 < (byte)*pcVar5;
    if (bVar1 != *pcVar5) {
LAB_005f410d:
      iVar3 = (1 - (uint)bVar6) - (uint)(bVar6 != 0);
      goto LAB_005f4112;
    }
    if (bVar1 == 0) break;
    bVar1 = pbVar2[1];
    bVar6 = bVar1 < (byte)pcVar5[1];
    if (bVar1 != pcVar5[1]) goto LAB_005f410d;
    pbVar2 = pbVar2 + 2;
    pcVar5 = pcVar5 + 2;
  } while (bVar1 != 0);
  iVar3 = 0;
LAB_005f4112:
  if (iVar3 == 0) {
    return 0x80000001;
  }
  pcVar5 = s_HKEY_CLASSES_ROOT_006ac218;
  pbVar2 = param_1;
  do {
    bVar1 = *pbVar2;
    bVar6 = bVar1 < (byte)*pcVar5;
    if (bVar1 != *pcVar5) {
LAB_005f4151:
      iVar3 = (1 - (uint)bVar6) - (uint)(bVar6 != 0);
      goto LAB_005f4156;
    }
    if (bVar1 == 0) break;
    bVar1 = pbVar2[1];
    bVar6 = bVar1 < (byte)pcVar5[1];
    if (bVar1 != pcVar5[1]) goto LAB_005f4151;
    pbVar2 = pbVar2 + 2;
    pcVar5 = pcVar5 + 2;
  } while (bVar1 != 0);
  iVar3 = 0;
LAB_005f4156:
  if (iVar3 == 0) {
    return 0x80000000;
  }
  pcVar5 = s_HKEY_USERS_006ac20c;
  do {
    bVar1 = *param_1;
    bVar6 = bVar1 < (byte)*pcVar5;
    if (bVar1 != *pcVar5) {
LAB_005f4195:
      iVar3 = (1 - (uint)bVar6) - (uint)(bVar6 != 0);
      goto LAB_005f419a;
    }
    if (bVar1 == 0) break;
    bVar1 = param_1[1];
    bVar6 = bVar1 < (byte)pcVar5[1];
    if (bVar1 != pcVar5[1]) goto LAB_005f4195;
    param_1 = param_1 + 2;
    pcVar5 = pcVar5 + 2;
  } while (bVar1 != 0);
  iVar3 = 0;
LAB_005f419a:
  if (iVar3 == 0) {
    uVar4 = 0x80000003;
  }
  return uVar4;
}

