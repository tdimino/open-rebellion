
void * __thiscall FUN_00616700(void *this,LPCSTR param_1,uint param_2,uint param_3,int param_4)

{
  undefined4 *puVar1;
  void *pvVar2;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065727d;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_4 != 0) {
    ExceptionList = &local_c;
    *(undefined **)this = &DAT_0066e570;
    FUN_006153b0((undefined4 *)((int)this + 0xc));
    local_4 = 0;
  }
  puVar1 = (undefined4 *)FUN_00618b70(0x54);
  local_4 = 1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_00616ed0(puVar1);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_006179e0(this,(int)puVar1,0);
  local_4 = 2;
  *(undefined ***)(*(int *)(*(int *)this + 4) + (int)this) = &PTR_LAB_0066e56c;
  *(undefined4 *)(*(int *)(*(int *)this + 4) + 0x1c + (int)this) = 1;
  pvVar2 = FUN_00618240(*(void **)(*(int *)(*(int *)this + 4) + 4 + (int)this),param_1,param_2 | 1,
                        param_3);
  if (pvVar2 == (void *)0x0) {
    *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
         *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 2;
  }
  ExceptionList = local_c;
  return this;
}

