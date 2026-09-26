
void * __thiscall FUN_00615c90(void *this,char *param_1,int param_2,byte param_3,int param_4)

{
  char cVar1;
  void *pvVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065718d;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_4 != 0) {
    ExceptionList = &local_c;
    *(undefined **)this = &DAT_0066e528;
    FUN_006153b0((undefined4 *)((int)this + 8));
    local_4 = 0;
  }
  pvVar2 = (void *)FUN_00618b70(100);
  local_4 = 1;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_00615530(pvVar2,param_1,param_2,(int)param_1);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00616210(this,(int)pvVar2,0);
  local_4 = 2;
  *(undefined ***)(*(int *)(*(int *)this + 4) + (int)this) = &PTR_LAB_0066e524;
  *(undefined4 *)(*(int *)(*(int *)this + 4) + 0x1c + (int)this) = 1;
  if ((param_3 & 0xc) != 0) {
    uVar3 = 0xffffffff;
    do {
      if (uVar3 == 0) break;
      uVar3 = uVar3 - 1;
      cVar1 = *param_1;
      param_1 = param_1 + 1;
    } while (cVar1 != '\0');
    FUN_006169a0(this,~uVar3 - 1,0);
  }
  ExceptionList = local_c;
  return this;
}

