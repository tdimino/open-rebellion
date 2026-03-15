
void __thiscall FUN_00600860(void *this,char *param_1)

{
  char cVar1;
  char *pcVar2;
  uint uVar3;
  uint uVar4;
  char *pcVar5;
  
  if (*(undefined **)((int)this + 0x84) != (undefined *)0x0) {
    FUN_00618b60(*(undefined **)((int)this + 0x84));
  }
  if (param_1 == (char *)0x0) {
    *(undefined4 *)((int)this + 0x84) = 0;
  }
  else {
    uVar3 = 0xffffffff;
    pcVar2 = param_1;
    do {
      if (uVar3 == 0) break;
      uVar3 = uVar3 - 1;
      cVar1 = *pcVar2;
      pcVar2 = pcVar2 + 1;
    } while (cVar1 != '\0');
    pcVar2 = (char *)FUN_00618b70(~uVar3);
    *(char **)((int)this + 0x84) = pcVar2;
    if (pcVar2 != (char *)0x0) {
      uVar3 = 0xffffffff;
      do {
        pcVar5 = param_1;
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        pcVar5 = param_1 + 1;
        cVar1 = *param_1;
        param_1 = pcVar5;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      pcVar5 = pcVar5 + -uVar3;
      for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        *(undefined4 *)pcVar2 = *(undefined4 *)pcVar5;
        pcVar5 = pcVar5 + 4;
        pcVar2 = pcVar2 + 4;
      }
      for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar2 = *pcVar5;
        pcVar5 = pcVar5 + 1;
        pcVar2 = pcVar2 + 1;
      }
      return;
    }
  }
  return;
}

