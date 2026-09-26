
bool __thiscall FUN_0040f5d0(void *this,int param_1)

{
  bool bVar1;
  ushort uVar2;
  int iVar3;
  int *piVar4;
  int iVar5;
  char *pcVar6;
  char *pcVar7;
  bool bVar8;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  undefined4 local_c;
  undefined4 local_8;
  undefined4 local_4;
  
  bVar1 = false;
  bVar8 = bVar1;
  if ((param_1 != 0) && (*(int *)((int)this + 0x188) != 0)) {
    uVar2 = FUN_005f50e0(param_1);
    if ((uint)uVar2 != *(uint *)(*(int *)(*(int *)((int)this + 0x188) + 0xa0) + 4)) {
      return true;
    }
    iVar3 = thunk_FUN_005f5060(param_1);
    for (; (bVar8 = bVar1, iVar3 != 0 && (bVar8 = true, !bVar1)); bVar1 = !bVar1) {
      piVar4 = (int *)FUN_00609eb0(*(int *)((int)this + 0x188));
      bVar8 = false;
      while ((bVar1 = bVar8, piVar4 != (int *)0x0 && (bVar1 = true, !bVar8))) {
        iVar5 = piVar4[0x15];
        local_20 = *(undefined4 *)(iVar5 + 0x1c);
        local_1c = *(undefined4 *)(iVar5 + 0x20);
        local_18 = *(undefined4 *)(iVar5 + 0x24);
        local_14 = *(undefined4 *)(iVar5 + 0x28);
        local_10 = *(undefined4 *)(iVar3 + 0x1c);
        local_c = *(undefined4 *)(iVar3 + 0x20);
        local_8 = *(undefined4 *)(iVar3 + 0x24);
        iVar5 = 0x10;
        local_4 = *(undefined4 *)(iVar3 + 0x28);
        bVar8 = true;
        pcVar6 = (char *)&local_10;
        pcVar7 = (char *)&local_20;
        do {
          if (iVar5 == 0) break;
          iVar5 = iVar5 + -1;
          bVar8 = *pcVar6 == *pcVar7;
          pcVar6 = pcVar6 + 1;
          pcVar7 = pcVar7 + 1;
        } while (bVar8);
        piVar4 = (int *)(**(code **)(*piVar4 + 0xc))();
      }
      iVar3 = *(int *)(iVar3 + 0x10);
    }
  }
  return bVar8;
}

