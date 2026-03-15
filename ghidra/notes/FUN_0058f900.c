
void __cdecl FUN_0058f900(byte *param_1)

{
  byte bVar1;
  bool bVar2;
  byte *pbVar3;
  int iVar4;
  int iVar5;
  byte *pbVar6;
  byte *pbVar7;
  bool bVar8;
  
  bVar2 = false;
  iVar5 = 0;
  if (0 < DAT_006bbe58) {
    pbVar7 = &DAT_006bbe60;
    do {
      pbVar3 = param_1;
      pbVar6 = pbVar7;
      if (bVar2) {
        return;
      }
      do {
        bVar1 = *pbVar3;
        bVar8 = bVar1 < *pbVar6;
        if (bVar1 != *pbVar6) {
LAB_0058f945:
          iVar4 = (1 - (uint)bVar8) - (uint)(bVar8 != 0);
          goto LAB_0058f94a;
        }
        if (bVar1 == 0) break;
        bVar1 = pbVar3[1];
        bVar8 = bVar1 < pbVar6[1];
        if (bVar1 != pbVar6[1]) goto LAB_0058f945;
        pbVar3 = pbVar3 + 2;
        pbVar6 = pbVar6 + 2;
      } while (bVar1 != 0);
      iVar4 = 0;
LAB_0058f94a:
      if (iVar4 == 0) {
        bVar2 = true;
      }
      iVar5 = iVar5 + 1;
      pbVar7 = pbVar7 + 0x2c;
    } while (iVar5 < DAT_006bbe58);
  }
  return;
}

