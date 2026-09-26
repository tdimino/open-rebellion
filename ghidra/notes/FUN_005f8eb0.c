
undefined4 __thiscall FUN_005f8eb0(void *this,int param_1,int param_2)

{
  uint uVar1;
  void *this_00;
  int iVar2;
  DWORD DVar3;
  int iVar4;
  uint uVar5;
  int iVar6;
  char cVar7;
  
  this_00 = (void *)FUN_005f7cc0();
  uVar1 = *(uint *)(param_1 + 0x20);
  if ((uVar1 != 0) && (uVar1 >> 0x1c == *(uint *)((int)this + 0x10c))) {
    iVar2 = FUN_005f5500(this_00,uVar1);
    if (iVar2 == 0) {
      return 1;
    }
    DVar3 = timeGetTime();
    iVar4 = FUN_005fb3a0(*(void **)(iVar2 + 0x1c),param_2);
    iVar6 = *(int *)((int)this + 0xc4) + 1;
    uVar5 = *(int *)((int)this + 200) + (DVar3 - iVar4);
    *(DWORD *)((int)this + 0xc0) = DVar3 - iVar4;
    *(int *)((int)this + 0xc4) = iVar6;
    *(uint *)((int)this + 200) = uVar5;
    if (iVar6 == 0) {
      *(undefined4 *)((int)this + 0xc4) = 1;
    }
    *(uint *)((int)this + 0xcc) = uVar5 / *(uint *)((int)this + 0xc4);
    iVar4 = FUN_005fb390(*(void **)(iVar2 + 0x1c),param_2);
    if (iVar4 != 3) {
      iVar4 = FUN_005fb390(*(void **)(iVar2 + 0x1c),param_2);
      if (iVar4 == 1) {
        iVar4 = FUN_005fb300(*(void **)(iVar2 + 0x1c),param_2);
        if (iVar4 == 1) {
          iVar4 = FUN_005fb340(*(int *)(iVar2 + 0x1c));
          if (iVar4 == 1) {
            uVar5 = FUN_005fad20(iVar2);
            iVar2 = FUN_005f5500((void *)((int)this + 0x84),uVar5);
            if ((iVar2 != 0) &&
               (cVar7 = *(char *)(iVar2 + 0x1d) + '\x01', *(char *)(iVar2 + 0x1d) = cVar7,
               *(char *)(iVar2 + 0x1c) <= cVar7)) {
              FUN_005f54a0((void *)((int)this + 0x84),uVar5);
            }
            FUN_005f54a0(this_00,uVar1);
          }
        }
        return 1;
      }
    }
  }
  return 0;
}

