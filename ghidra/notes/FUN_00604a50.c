
void __thiscall FUN_00604a50(void *this,HDC param_1)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  uint local_4;
  
  if ((*(int *)((int)this + 0xa8) != 0) && (*(void **)((int)this + 0xb0) != (void *)0x0)) {
    FUN_005fcc30(*(void **)((int)this + 0xb0),*(uint **)((int)this + 0xb4),0,0,
                 *(int *)((int)this + 0x94),*(uint *)((int)this + 0x98),(undefined4 *)0x0,0);
    FUN_005fd0f0(*(void **)((int)this + 0xa8),*(uint **)((int)this + 0xb4),0,0);
  }
  if (*(int *)((int)this + 0xa4) == 1) {
    iVar3 = *(int *)((int)this + 0xd0);
    iVar2 = *(int *)((int)this + 0xcc) - *(int *)((int)this + 0xa0) / 2;
    uVar1 = *(uint *)((int)this + 0xd8);
    local_4 = uVar1;
    if (uVar1 == 0) {
      local_4 = *(uint *)((int)this + 0x3c);
    }
    local_4 = ((uVar1 - *(int *)((int)this + 0xcc)) * *(int *)((int)this + 0xe0)) / local_4;
  }
  else {
    if (*(int *)((int)this + 0xa4) != 2) {
      iVar3 = 0;
      iVar2 = 0;
      goto LAB_00604b3a;
    }
    uVar1 = *(uint *)((int)this + 0xd4);
    iVar2 = *(int *)((int)this + 0xd0);
    iVar3 = *(int *)((int)this + 200) - *(int *)((int)this + 0x9c) / 2;
    local_4 = uVar1;
    if (uVar1 == 0) {
      local_4 = *(uint *)((int)this + 0x38);
    }
    local_4 = ((*(int *)((int)this + 200) - uVar1) * *(int *)((int)this + 0xe0)) / local_4;
  }
  *(uint *)((int)this + 0xdc) = local_4;
LAB_00604b3a:
  if (*(void **)((int)this + 0xac) != (void *)0x0) {
    FUN_005fd0f0(*(void **)((int)this + 0xac),*(uint **)((int)this + 0xb4),iVar3,iVar2);
    FUN_005fc140(*(void **)((int)this + 0xb4),param_1,'@',0xcc0020,0,0,0,0,0,0);
  }
  return;
}

