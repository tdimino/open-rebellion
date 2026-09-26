
int __thiscall FUN_0052e580(void *this,uint param_1,void *param_2)

{
  uint *puVar1;
  int iVar2;
  int iVar3;
  bool bVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  void *pvVar9;
  uint *puVar10;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  uVar5 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006451f8;
  local_c = ExceptionList;
  iVar3 = 0;
  bVar4 = true;
  if (param_1 == 0x28) {
    iVar3 = *(int *)((int)this + 0x9c);
  }
  else if (param_1 == 0x29) {
    iVar3 = *(int *)((int)this + 0xa0);
  }
  else if (param_1 == 0x2a) {
    iVar3 = *(int *)((int)this + 0xa4);
  }
  else {
    bVar4 = false;
  }
  param_1 = 0;
  iVar2 = 0;
  if (bVar4) {
    ExceptionList = &local_c;
    local_10 = this;
    puVar1 = FUN_004025b0(this,(uint *)&local_10);
    puVar10 = &param_1;
    iVar8 = 0;
    iVar7 = 1;
    iVar6 = 1;
    local_4 = 0;
    pvVar9 = param_2;
    iVar2 = FUN_00530a20(puVar1);
    iVar2 = FUN_0051cb20(iVar2,uVar5,iVar3,iVar6,iVar7,iVar8,pvVar9,(int *)puVar10);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar2;
}

