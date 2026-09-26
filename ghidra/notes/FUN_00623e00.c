
void __cdecl FUN_00623e00(byte *param_1)

{
  byte bVar1;
  byte *pbVar2;
  int iVar3;
  byte *pbVar4;
  bool bVar5;
  byte local_8 [8];
  
  if ((param_1 == (byte *)0x0) || (*param_1 == 0)) {
LAB_00623eac:
    iVar3 = (*DAT_006bec04)(DAT_006bec00,0x1004,local_8,8);
    if (iVar3 == 0) {
      return;
    }
  }
  else {
    pbVar4 = &DAT_0066f29c;
    pbVar2 = param_1;
    do {
      bVar1 = *pbVar2;
      bVar5 = bVar1 < *pbVar4;
      if (bVar1 != *pbVar4) {
LAB_00623e4a:
        iVar3 = (1 - (uint)bVar5) - (uint)(bVar5 != 0);
        goto LAB_00623e4f;
      }
      if (bVar1 == 0) break;
      bVar1 = pbVar2[1];
      bVar5 = bVar1 < pbVar4[1];
      if (bVar1 != pbVar4[1]) goto LAB_00623e4a;
      pbVar2 = pbVar2 + 2;
      pbVar4 = pbVar4 + 2;
    } while (bVar1 != 0);
    iVar3 = 0;
LAB_00623e4f:
    if (iVar3 == 0) goto LAB_00623eac;
    pbVar4 = &DAT_0066f298;
    pbVar2 = param_1;
    do {
      bVar1 = *pbVar2;
      bVar5 = bVar1 < *pbVar4;
      if (bVar1 != *pbVar4) {
LAB_00623e82:
        iVar3 = (1 - (uint)bVar5) - (uint)(bVar5 != 0);
        goto LAB_00623e87;
      }
      if (bVar1 == 0) break;
      bVar1 = pbVar2[1];
      bVar5 = bVar1 < pbVar4[1];
      if (bVar1 != pbVar4[1]) goto LAB_00623e82;
      pbVar2 = pbVar2 + 2;
      pbVar4 = pbVar4 + 2;
    } while (bVar1 != 0);
    iVar3 = 0;
LAB_00623e87:
    if (iVar3 != 0) goto LAB_00623ed3;
    iVar3 = (*DAT_006bec04)(DAT_006bec00,0xb,local_8,8);
    if (iVar3 == 0) {
      return;
    }
  }
  param_1 = local_8;
LAB_00623ed3:
  FUN_00619560(param_1);
  return;
}

