
bool __thiscall FUN_005fcaa0(void *this,int param_1,uint param_2)

{
  short sVar1;
  byte bVar2;
  int iVar3;
  int iVar4;
  byte *pbVar5;
  byte *pbVar6;
  uint uVar7;
  bool bVar8;
  
  bVar8 = false;
  iVar3 = FUN_005fc0f0(this);
  iVar4 = ((iVar3 < 1) - 1 & 0xfffffffe) + 1;
  if (iVar4 == -1) {
    uVar7 = iVar3 - 1;
  }
  else {
    uVar7 = 0;
  }
  pbVar5 = *(byte **)((int)this + 4);
  if (iVar3 < 1) {
    iVar3 = -iVar3;
  }
  if (((((int)param_2 < iVar3) && (0 < (int)param_2)) &&
      (iVar3 = FUN_005fc0e0(this), param_1 < iVar3)) && (0 < param_1)) {
    while (uVar7 != param_2) {
      sVar1 = *(short *)pbVar5;
      if (sVar1 == 0) {
        uVar7 = uVar7 + iVar4;
      }
      else {
        if (sVar1 == 0x100) {
          return false;
        }
        if (sVar1 == 0x200) {
          uVar7 = uVar7 + (uint)pbVar5[3] * iVar4;
          pbVar5 = pbVar5 + 2;
        }
        else if (*pbVar5 == 0) {
          pbVar5 = pbVar5 + ((int)(pbVar5[1] + 1) >> 1) * 2;
        }
      }
      pbVar5 = pbVar5 + 2;
    }
    iVar3 = 0;
    if (0 < param_1) {
      do {
        sVar1 = *(short *)pbVar5;
        pbVar6 = pbVar5;
        if ((sVar1 == 0) || (sVar1 == 0x100)) {
LAB_005fcc05:
          iVar3 = param_1;
        }
        else {
          if (sVar1 == 0x200) {
            pbVar6 = pbVar5 + 2;
            iVar3 = iVar3 + (uint)*pbVar6;
            if (pbVar5[3] == 0) goto LAB_005fcc07;
            goto LAB_005fcc05;
          }
          bVar2 = *pbVar5;
          if (bVar2 == 0) {
            pbVar6 = pbVar5 + 2;
            bVar2 = pbVar5[1];
            for (; (bVar2 != 0 && (bVar2 = bVar2 - 1, iVar3 < param_1)); iVar3 = iVar3 + 1) {
              bVar8 = *pbVar6 != *(byte *)((int)this + 8);
              pbVar6 = pbVar6 + 1;
            }
            if ((pbVar5[1] & 1) != 0) goto LAB_005fcbf2;
          }
          else {
            pbVar6 = pbVar5 + 1;
            param_2 = (uint)bVar2;
            if (bVar2 == 0) {
LAB_005fcbf2:
              pbVar6 = pbVar6 + 1;
            }
            else {
              bVar8 = *pbVar6 != *(byte *)((int)this + 8);
              pbVar6 = pbVar5 + 2;
              iVar3 = iVar3 + param_2;
            }
          }
        }
LAB_005fcc07:
        pbVar5 = pbVar6;
      } while (iVar3 < param_1);
    }
  }
  return bVar8;
}

