
int __cdecl FUN_0061a620(byte *param_1,char *param_2,int param_3)

{
  char cVar1;
  LONG *pLVar2;
  byte bVar3;
  ushort uVar4;
  uint uVar5;
  int iVar6;
  uint uVar7;
  bool bVar8;
  uint uVar9;
  
  pLVar2 = lpAddend_006bfe48;
  iVar6 = 0;
  if (param_3 != 0) {
    if (DAT_006be840 == 0) {
      do {
        bVar3 = *param_1;
        cVar1 = *param_2;
        uVar4 = CONCAT11(bVar3,cVar1);
        if (bVar3 == 0) break;
        uVar4 = CONCAT11(bVar3,cVar1);
        uVar7 = (uint)uVar4;
        if (cVar1 == '\0') break;
        param_1 = param_1 + 1;
        param_2 = param_2 + 1;
        if ((0x40 < bVar3) && (bVar3 < 0x5b)) {
          uVar7 = (uint)CONCAT11(bVar3 + 0x20,cVar1);
        }
        uVar4 = (ushort)uVar7;
        bVar3 = (byte)uVar7;
        if ((0x40 < bVar3) && (bVar3 < 0x5b)) {
          uVar4 = (ushort)CONCAT31((int3)(uVar7 >> 8),bVar3 + 0x20);
        }
        bVar3 = (byte)(uVar4 >> 8);
        bVar8 = bVar3 < (byte)uVar4;
        if (bVar3 != (byte)uVar4) goto LAB_0061a67f;
        param_3 = param_3 + -1;
      } while (param_3 != 0);
      iVar6 = 0;
      bVar3 = (byte)(uVar4 >> 8);
      bVar8 = bVar3 < (byte)uVar4;
      if (bVar3 != (byte)uVar4) {
LAB_0061a67f:
        iVar6 = -1;
        if (!bVar8) {
          iVar6 = 1;
        }
      }
    }
    else {
      LOCK();
      lpAddend_006bfe48 = (LONG *)((int)lpAddend_006bfe48 + 1);
      UNLOCK();
      bVar8 = 0 < DAT_006bfe44;
      if (bVar8) {
        LOCK();
        UNLOCK();
        lpAddend_006bfe48 = pLVar2;
        FUN_0061d9d0(0x13);
      }
      uVar9 = (uint)bVar8;
      uVar7 = 0;
      uVar5 = 0;
      do {
        uVar5 = CONCAT31((int3)(uVar5 >> 8),*param_1);
        uVar7 = CONCAT31((int3)(uVar7 >> 8),*param_2);
        if ((uVar5 == 0) || (uVar7 == 0)) break;
        param_1 = param_1 + 1;
        param_2 = param_2 + 1;
        uVar7 = FUN_00622070(uVar7);
        uVar5 = FUN_00622070(uVar5);
        bVar8 = uVar5 < uVar7;
        if (uVar5 != uVar7) goto LAB_0061a6f5;
        param_3 = param_3 + -1;
      } while (param_3 != 0);
      iVar6 = 0;
      bVar8 = uVar5 < uVar7;
      if (uVar5 != uVar7) {
LAB_0061a6f5:
        iVar6 = -1;
        if (!bVar8) {
          iVar6 = 1;
        }
      }
      if (uVar9 == 0) {
        LOCK();
        lpAddend_006bfe48 = (LONG *)((int)lpAddend_006bfe48 + -1);
        UNLOCK();
      }
      else {
        FUN_0061da50(0x13);
      }
    }
  }
  return iVar6;
}

