
void __thiscall FUN_005f3ad0(void *this,char *param_1,size_t param_2)

{
  char cVar1;
  ushort uVar2;
  uint uVar3;
  char *_Dest;
  int iVar4;
  char *pcVar5;
  uint uVar6;
  char *pcVar7;
  char *pcVar8;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006555d8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f3de0(local_18,(int)this);
  local_4 = 0;
  uVar3 = FUN_005f3070((int)this);
  if ((int)(uVar3 & 0xffff) < (int)param_2) {
    param_2 = uVar3 & 0xffff;
  }
  uVar3 = FUN_005f3630(*(char **)((int)this + 4));
  uVar2 = FUN_00479ac0();
  _Dest = (char *)FUN_00618b70((uVar3 & 0xffff) + 2 + (uint)uVar2);
  if (_Dest != (char *)0x0) {
    uVar3 = 0xffffffff;
    pcVar5 = &DAT_006b120c;
    do {
      pcVar8 = pcVar5;
      if (uVar3 == 0) break;
      uVar3 = uVar3 - 1;
      pcVar8 = pcVar5 + 1;
      cVar1 = *pcVar5;
      pcVar5 = pcVar8;
    } while (cVar1 != '\0');
    uVar3 = ~uVar3;
    pcVar5 = pcVar8 + -uVar3;
    pcVar8 = _Dest;
    for (uVar6 = uVar3 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
      *(undefined4 *)pcVar8 = *(undefined4 *)pcVar5;
      pcVar5 = pcVar5 + 4;
      pcVar8 = pcVar8 + 4;
    }
    for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
      *pcVar8 = *pcVar5;
      pcVar5 = pcVar5 + 1;
      pcVar8 = pcVar8 + 1;
    }
    if (*(char **)((int)this + 4) != (char *)0x0) {
      _strncat(_Dest,*(char **)((int)this + 4),param_2);
    }
    _strncat(_Dest,param_1,1);
    iVar4 = FUN_00583c40((int)local_18);
    if (iVar4 != 0) {
      for (; param_2 != 0; param_2 = param_2 - 1) {
        FUN_005f3e50((int)local_18);
      }
      pcVar5 = (char *)FUN_00583c40((int)local_18);
      uVar3 = 0xffffffff;
      do {
        pcVar8 = pcVar5;
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        pcVar8 = pcVar5 + 1;
        cVar1 = *pcVar5;
        pcVar5 = pcVar8;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar4 = -1;
      pcVar5 = _Dest;
      do {
        pcVar7 = pcVar5;
        if (iVar4 == 0) break;
        iVar4 = iVar4 + -1;
        pcVar7 = pcVar5 + 1;
        cVar1 = *pcVar5;
        pcVar5 = pcVar7;
      } while (cVar1 != '\0');
      pcVar5 = pcVar8 + -uVar3;
      pcVar8 = pcVar7 + -1;
      for (uVar6 = uVar3 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(undefined4 *)pcVar8 = *(undefined4 *)pcVar5;
        pcVar5 = pcVar5 + 4;
        pcVar8 = pcVar8 + 4;
      }
      for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar8 = *pcVar5;
        pcVar5 = pcVar5 + 1;
        pcVar8 = pcVar8 + 1;
      }
    }
    FUN_00618b60(*(undefined **)((int)this + 4));
    *(char **)((int)this + 4) = _Dest;
  }
  local_4 = 0xffffffff;
  FUN_005f3e00(local_18);
  ExceptionList = local_c;
  return;
}

