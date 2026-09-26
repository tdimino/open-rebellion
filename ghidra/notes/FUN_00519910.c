
void __thiscall FUN_00519910(void *this,void *param_1)

{
  bool bVar1;
  void *pvVar2;
  int *piVar3;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar2 = param_1;
  puStack_8 = &LAB_00642d08;
  local_c = ExceptionList;
  local_10 = 0xf2000002;
  local_4 = 0;
  ExceptionList = &local_c;
  piVar3 = FUN_004f7d50(this,&local_10,3,param_1,1,1);
  *(int **)((int)this + 0xb0) = piVar3;
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = (void *)0xf8000008;
  local_4 = 1;
  piVar3 = FUN_004f7d50(this,(uint *)&param_1,3,pvVar2,1,1);
  *(int **)((int)this + 0xb8) = piVar3;
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = (void *)0xf9000009;
  local_4 = 2;
  piVar3 = FUN_004f7d50(this,(uint *)&param_1,3,pvVar2,1,1);
  *(int **)((int)this + 0xbc) = piVar3;
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = (void *)0xfa00000a;
  local_4 = 3;
  piVar3 = FUN_004f7d50(this,(uint *)&param_1,3,pvVar2,1,1);
  *(int **)((int)this + 0xc0) = piVar3;
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = (void *)0xf3000003;
  local_4 = 4;
  piVar3 = FUN_004f7d50(this,(uint *)&param_1,1,pvVar2,1,1);
  *(int **)((int)this + 0xc4) = piVar3;
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = (void *)0xf3000003;
  local_4 = 5;
  piVar3 = FUN_004f7d50(this,(uint *)&param_1,2,pvVar2,1,1);
  *(int **)((int)this + 200) = piVar3;
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = (void *)0x98000440;
  local_4 = 6;
  piVar3 = FUN_004f7d50(this,(uint *)&param_1,1,pvVar2,1,1);
  *(int **)((int)this + 0xcc) = piVar3;
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = (void *)0x98000441;
  local_4 = 7;
  piVar3 = FUN_004f7d50(this,(uint *)&param_1,3,pvVar2,1,1);
  *(int **)((int)this + 0xd0) = piVar3;
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = (void *)0x98000442;
  local_4 = 8;
  piVar3 = FUN_004f7d50(this,(uint *)&param_1,3,pvVar2,1,1);
  *(int **)((int)this + 0xd4) = piVar3;
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = (void *)0x98000480;
  local_4 = 9;
  piVar3 = FUN_004f7d50(this,(uint *)&param_1,2,pvVar2,1,1);
  *(int **)((int)this + 0xd8) = piVar3;
  local_4 = 0xffffffff;
  FUN_00619730();
  param_1 = (void *)0x98000481;
  local_4 = 10;
  piVar3 = FUN_004f7d50(this,(uint *)&param_1,2,pvVar2,1,1);
  *(int **)((int)this + 0xdc) = piVar3;
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((((((*(int *)((int)this + 0xb0) == 0) || (*(int *)((int)this + 0xb8) == 0)) ||
        (*(int *)((int)this + 0xbc) == 0)) ||
       ((*(int *)((int)this + 0xc0) == 0 || (*(int *)((int)this + 0xc4) == 0)))) ||
      ((*(int *)((int)this + 200) == 0 ||
       ((*(int *)((int)this + 0xcc) == 0 || (*(int *)((int)this + 0xd0) == 0)))))) ||
     ((*(int *)((int)this + 0xd4) == 0 ||
      ((*(int *)((int)this + 0xd8) == 0 || (*(int *)((int)this + 0xdc) == 0)))))) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (bVar1) {
    *(undefined4 *)(*(int *)((int)this + 0xc0) + 0x5c) = *(undefined4 *)((int)this + 0x5c);
  }
  ExceptionList = local_c;
  return;
}

