
void __thiscall FUN_0045d090(void *this,void *param_1)

{
  void *pvVar1;
  uint *puVar2;
  void *pvVar3;
  uint uVar4;
  int iVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630c18;
  local_c = ExceptionList;
  if (param_1 != (void *)0x0) {
    ExceptionList = &local_c;
    puVar2 = FUN_004025b0(param_1,(uint *)&param_1);
    local_4 = 0;
    pvVar3 = (void *)FUN_0060a860((void *)((int)this + 0x174),(ushort)*puVar2 | 0x400000);
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar4 = FUN_004a1f60((int)pvVar1,*(int *)(*(int *)((int)this + 0x194) + 0x9c));
    if ((uVar4 == 0) || (uVar4 == 3)) {
      iVar5 = 0;
    }
    else {
      iVar5 = 1;
    }
    FUN_0045d140(this,pvVar3,uVar4,iVar5);
  }
  ExceptionList = local_c;
  return;
}

