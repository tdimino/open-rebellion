
undefined4 __cdecl FUN_00585070(int *param_1)

{
  char cVar1;
  byte bVar2;
  byte *pbVar3;
  int iVar4;
  int *this;
  uint uVar5;
  uint uVar6;
  byte *pbVar7;
  char *pcVar8;
  char *pcVar9;
  bool bVar10;
  int local_7c;
  byte local_78 [8];
  int local_70 [5];
  undefined4 local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f418;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00615db0(local_70,1);
  uVar5 = 0xffffffff;
  pcVar8 = &DAT_006b120c;
  do {
    pcVar9 = pcVar8;
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    pcVar9 = pcVar8 + 1;
    cVar1 = *pcVar8;
    pcVar8 = pcVar9;
  } while (cVar1 != '\0');
  uVar5 = ~uVar5;
  pbVar3 = (byte *)(pcVar9 + -uVar5);
  pbVar7 = local_78;
  for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
    *(undefined4 *)pbVar7 = *(undefined4 *)pbVar3;
    pbVar3 = pbVar3 + 4;
    pbVar7 = pbVar7 + 4;
  }
  local_4 = 0;
  for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
    *pbVar7 = *pbVar3;
    pbVar3 = pbVar3 + 1;
    pbVar7 = pbVar7 + 1;
  }
  iVar4 = *(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1);
  while (iVar4 == 0) {
    local_7c = 0;
    pbVar7 = &DAT_0066a1ac;
    pbVar3 = local_78;
    do {
      bVar2 = *pbVar3;
      bVar10 = bVar2 < *pbVar7;
      if (bVar2 != *pbVar7) {
LAB_0058510c:
        iVar4 = (1 - (uint)bVar10) - (uint)(bVar10 != 0);
        goto LAB_00585111;
      }
      if (bVar2 == 0) break;
      bVar2 = pbVar3[1];
      bVar10 = bVar2 < pbVar7[1];
      if (bVar2 != pbVar7[1]) goto LAB_0058510c;
      pbVar3 = pbVar3 + 2;
      pbVar7 = pbVar7 + 2;
    } while (bVar2 != 0);
    iVar4 = 0;
LAB_00585111:
    if (iVar4 == 0) break;
    this = FUN_00584e60(param_1,local_70,&local_7c);
    if (*(int *)((int)this + *(int *)(*this + 4) + 0x34) < 0) {
      FUN_00615ee0((LPCRITICAL_SECTION)((int)this + *(int *)(*this + 4) + 0x38));
    }
    this[1] = this[1] + 1;
    FUN_00617800(this,(int)local_78,5,0);
    if (*(int *)(*(int *)(*this + 4) + 0x34 + (int)this) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)this + *(int *)(*this + 4) + 0x38));
    }
    iVar4 = *(int *)(*(int *)(*param_1 + 4) + 8 + (int)param_1);
  }
  local_7c = 0;
  FUN_00584fd0(param_1,&local_7c);
  local_4 = 0xffffffff;
  FUN_00615ea0((int)local_5c);
  FUN_00615440(local_5c);
  ExceptionList = pvStack_c;
  return 1;
}

