
void * __thiscall FUN_00585dc0(void *this,uint *param_1)

{
  void *this_00;
  uint uVar1;
  uint uVar2;
  bool bVar3;
  uint *puVar4;
  uint *puVar5;
  void *local_4;
  
  puVar4 = param_1;
  this_00 = *(void **)((int)this + 4);
  bVar3 = false;
  local_4 = this;
  while ((this_00 != (void *)0x0 && (!bVar3))) {
    puVar5 = (uint *)FUN_005843d0(this_00,(uint *)&param_1);
    uVar1 = *puVar4;
    uVar2 = *puVar5;
    FUN_00619730();
    if (uVar2 < uVar1) {
      this_00 = *(void **)((int)this_00 + 8);
    }
    else {
      puVar5 = (uint *)FUN_005843d0(this_00,(uint *)&local_4);
      uVar1 = *puVar4;
      uVar2 = *puVar5;
      FUN_00619730();
      if (uVar1 < uVar2) {
        this_00 = *(void **)((int)this_00 + 4);
      }
      else {
        bVar3 = true;
      }
    }
  }
  return this_00;
}

