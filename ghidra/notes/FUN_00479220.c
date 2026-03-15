
void * __thiscall FUN_00479220(void *this,void *param_1,undefined4 *param_2)

{
  int iVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  void *pvVar5;
  undefined4 *puVar6;
  uint local_1c;
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633377;
  local_c = ExceptionList;
  bVar2 = false;
  local_10 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_1c);
  uVar3 = *(uint *)((int)this + 0x40) >> 0x18;
  local_4 = 1;
  local_18 = (undefined1 *)0x90;
  local_14 = 0x98;
  if ((0x8f < uVar3) && (uVar3 < 0x98)) {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    iVar1 = *(int *)((int)this + 0x58);
    local_18 = &stack0xffffffd0;
    FUN_004f26d0(&stack0xffffffd0,(undefined4 *)((int)this + 0x40));
    pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if ((pvVar4 != (void *)0x0) && ((*(byte *)((int)pvVar4 + 0x28) & 2) == 0)) {
      FUN_004f26d0(&local_1c,(undefined4 *)((int)this + 0x40));
    }
  }
  local_18 = (undefined1 *)0x90;
  local_14 = 0x98;
  if ((local_1c >> 0x18 < 0x90) || (0x97 < local_1c >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (!bVar2) {
    pvVar5 = FUN_0041a340(*(void **)((int)this + 0x58),(undefined4 *)((int)this + 0x34),0x80000000,0
                          ,1,2,0,0,7,&DAT_00000002);
    pvVar4 = (void *)((int)this + 0x48);
    FUN_00435790(pvVar4,(int)pvVar5);
    puVar6 = (undefined4 *)FUN_004357b0(pvVar4,&local_18);
    local_4._0_1_ = 2;
    FUN_004f26d0(&local_1c,puVar6);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    FUN_005f58b0(pvVar4);
  }
  local_18 = (undefined1 *)0x90;
  local_14 = 0x98;
  if ((local_1c >> 0x18 < 0x90) || (0x97 < local_1c >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (!bVar2) {
    puVar6 = FUN_0041a400(*(void **)((int)this + 0x58),(undefined4 *)((int)this + 0x34),param_2,0,0,
                          1,2,0,0,(undefined4 *)&DAT_00000002);
    pvVar4 = (void *)((int)this + 0x48);
    FUN_00435790(pvVar4,(int)puVar6);
    puVar6 = (undefined4 *)FUN_004357b0(pvVar4,&param_2);
    local_4._0_1_ = 3;
    FUN_004f26d0(&local_1c,puVar6);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    FUN_005f58b0(pvVar4);
  }
  local_18 = (undefined1 *)0x90;
  local_14 = 0x98;
  if ((local_1c >> 0x18 < 0x90) || (0x97 < local_1c >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (!bVar2) {
    puVar6 = (undefined4 *)
             FUN_0041b2b0(*(void **)((int)this + 0x58),&param_2,(uint *)((int)this + 0x34),0,0,1,2,0
                          ,0);
    local_4._0_1_ = 4;
    FUN_004f26d0(&local_1c,puVar6);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
  FUN_004f26d0(param_1,&local_1c);
  local_10 = 1;
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

