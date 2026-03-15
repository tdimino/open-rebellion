
void * __thiscall FUN_005f3240(void *this,char *param_1)

{
  char cVar1;
  ushort uVar2;
  char *_Dest;
  uint uVar3;
  uint uVar4;
  char *pcVar5;
  char *pcVar6;
  
  uVar2 = FUN_005f3040((int)this);
  _Dest = (char *)FUN_00618b70(uVar2 + 4);
  *_Dest = '\0';
  if (*(char **)((int)this + 4) != (char *)0x0) {
    uVar3 = 0xffffffff;
    pcVar5 = *(char **)((int)this + 4);
    do {
      pcVar6 = pcVar5;
      if (uVar3 == 0) break;
      uVar3 = uVar3 - 1;
      pcVar6 = pcVar5 + 1;
      cVar1 = *pcVar5;
      pcVar5 = pcVar6;
    } while (cVar1 != '\0');
    uVar3 = ~uVar3;
    pcVar5 = pcVar6 + -uVar3;
    pcVar6 = _Dest;
    for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
      *(undefined4 *)pcVar6 = *(undefined4 *)pcVar5;
      pcVar5 = pcVar5 + 4;
      pcVar6 = pcVar6 + 4;
    }
    for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
      *pcVar6 = *pcVar5;
      pcVar5 = pcVar5 + 1;
      pcVar6 = pcVar6 + 1;
    }
    FUN_00618b60(*(undefined **)((int)this + 4));
  }
  _strncat(_Dest,param_1,1);
  *(char **)((int)this + 4) = _Dest;
  return this;
}

