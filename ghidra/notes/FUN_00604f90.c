
void __thiscall FUN_00604f90(void *this,char *param_1)

{
  char cVar1;
  uint uVar2;
  int iVar3;
  uint uVar4;
  char *pcVar5;
  char local_104 [260];
  
  if (param_1 == (char *)0x0) {
    FUN_00605070(this);
    *(undefined4 *)((int)this + 0xd0) = 0;
    *(undefined4 *)((int)this + 0xd4) = 0;
    *(undefined4 *)((int)this + 0xcc) = 0xffffffff;
    return;
  }
  uVar4 = *(uint *)((int)this + 0xa4);
  if (uVar4 != 0) {
    uVar2 = 0xffffffff;
    pcVar5 = param_1;
    do {
      if (uVar2 == 0) break;
      uVar2 = uVar2 - 1;
      cVar1 = *pcVar5;
      pcVar5 = pcVar5 + 1;
    } while (cVar1 != '\0');
    if (uVar4 <= ~uVar2 - 1) {
      pcVar5 = local_104;
      for (iVar3 = 0x41; iVar3 != 0; iVar3 = iVar3 + -1) {
        pcVar5[0] = '\0';
        pcVar5[1] = '\0';
        pcVar5[2] = '\0';
        pcVar5[3] = '\0';
        pcVar5 = pcVar5 + 4;
      }
      pcVar5 = local_104;
      for (uVar2 = uVar4 - 1 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
        *(undefined4 *)pcVar5 = *(undefined4 *)param_1;
        param_1 = param_1 + 4;
        pcVar5 = pcVar5 + 4;
      }
      for (uVar4 = uVar4 - 1 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar5 = *param_1;
        param_1 = param_1 + 1;
        pcVar5 = pcVar5 + 1;
      }
      param_1 = local_104;
    }
  }
  FUN_005f35e0((void *)((int)this + 0x98),param_1);
  uVar4 = FUN_005f3070((int)this + 0x98);
  *(undefined4 *)((int)this + 0xcc) = 0xffffffff;
  *(uint *)((int)this + 0xd0) = uVar4 & 0xffff;
  if (*(int *)((int)this + 0x18) != 0) {
    FUN_00605fc0(this);
    FUN_006060f0(this);
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
  }
  return;
}

