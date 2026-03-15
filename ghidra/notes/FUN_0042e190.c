
int __thiscall FUN_0042e190(void *this,uint *param_1)

{
  int iVar1;
  uint uVar2;
  bool bVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  void *local_4;
  
  local_4 = this;
  FUN_0042dbe0(&local_4);
  iVar1 = *(int *)((int)this + 4);
  bVar3 = false;
  uVar4 = *param_1 & 0xffffff;
  uVar6 = *param_1 >> 0x18;
  while ((iVar1 != 0 && (!bVar3))) {
    uVar2 = *(uint *)(iVar1 + 0x2c);
    uVar5 = uVar2 >> 0x18;
    if ((uVar6 < uVar5) || ((uVar6 == uVar5 && (uVar4 < (uVar2 & 0xffffff))))) {
      iVar1 = *(int *)(iVar1 + 4);
    }
    else if ((uVar5 < uVar6) || ((uVar6 == uVar5 && ((uVar2 & 0xffffff) < uVar4)))) {
      iVar1 = *(int *)(iVar1 + 8);
    }
    else {
      bVar3 = true;
    }
  }
  FUN_00619730();
  return iVar1;
}

