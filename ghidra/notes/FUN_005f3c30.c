
void __thiscall FUN_005f3c30(void *this,size_t param_1,int param_2)

{
  char cVar1;
  ushort uVar2;
  uint uVar3;
  char *_Dest;
  char *pcVar4;
  uint uVar5;
  int iVar6;
  char *pcVar7;
  char *pcVar8;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006555f8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar3 = FUN_005f3070((int)this);
  FUN_005f3de0(local_18,(int)this);
  local_4 = 0;
  if ((((-1 < (int)param_1) && ((int)param_1 < (int)(uVar3 & 0xffff))) && ((int)param_1 < param_2))
     && (param_2 <= (int)(uVar3 & 0xffff))) {
    uVar3 = FUN_005f3630(*(char **)((int)this + 4));
    uVar2 = FUN_00479ac0();
    _Dest = (char *)FUN_00618b70((uVar3 & 0xffff) + (uint)uVar2);
    uVar3 = 0xffffffff;
    pcVar4 = &DAT_006b120c;
    do {
      pcVar8 = pcVar4;
      if (uVar3 == 0) break;
      uVar3 = uVar3 - 1;
      pcVar8 = pcVar4 + 1;
      cVar1 = *pcVar4;
      pcVar4 = pcVar8;
    } while (cVar1 != '\0');
    uVar3 = ~uVar3;
    pcVar4 = pcVar8 + -uVar3;
    pcVar8 = _Dest;
    for (uVar5 = uVar3 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
      *(undefined4 *)pcVar8 = *(undefined4 *)pcVar4;
      pcVar4 = pcVar4 + 4;
      pcVar8 = pcVar8 + 4;
    }
    for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
      *pcVar8 = *pcVar4;
      pcVar4 = pcVar4 + 1;
      pcVar8 = pcVar8 + 1;
    }
    if (0 < (int)param_1) {
      _strncat(_Dest,*(char **)((int)this + 4),param_1);
    }
    for (; param_2 != 0; param_2 = param_2 + -1) {
      FUN_005f3e50((int)local_18);
    }
    pcVar4 = (char *)FUN_00583c40((int)local_18);
    uVar3 = 0xffffffff;
    do {
      pcVar8 = pcVar4;
      if (uVar3 == 0) break;
      uVar3 = uVar3 - 1;
      pcVar8 = pcVar4 + 1;
      cVar1 = *pcVar4;
      pcVar4 = pcVar8;
    } while (cVar1 != '\0');
    uVar3 = ~uVar3;
    iVar6 = -1;
    pcVar4 = _Dest;
    do {
      pcVar7 = pcVar4;
      if (iVar6 == 0) break;
      iVar6 = iVar6 + -1;
      pcVar7 = pcVar4 + 1;
      cVar1 = *pcVar4;
      pcVar4 = pcVar7;
    } while (cVar1 != '\0');
    pcVar4 = pcVar8 + -uVar3;
    pcVar8 = pcVar7 + -1;
    for (uVar5 = uVar3 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
      *(undefined4 *)pcVar8 = *(undefined4 *)pcVar4;
      pcVar4 = pcVar4 + 4;
      pcVar8 = pcVar8 + 4;
    }
    for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
      *pcVar8 = *pcVar4;
      pcVar4 = pcVar4 + 1;
      pcVar8 = pcVar8 + 1;
    }
    FUN_00618b60(*(undefined **)((int)this + 4));
    *(char **)((int)this + 4) = _Dest;
  }
  local_4 = 0xffffffff;
  FUN_005f3e00(local_18);
  ExceptionList = local_c;
  return;
}

