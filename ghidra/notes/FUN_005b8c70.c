
void __thiscall FUN_005b8c70(void *this,int *param_1)

{
  void *this_00;
  int unaff_ESI;
  int local_10;
  int iStack_c;
  int iStack_8;
  
  FUN_005c5fb0(*(void **)((int)this + 0x1c),(int)param_1);
                    /* WARNING: Load size is inaccurate */
  (**(code **)(*this + 4))(&local_10);
  param_1[0xc] = unaff_ESI;
  param_1[0xd] = local_10;
  param_1[0xe] = iStack_c;
  param_1[0xf] = iStack_8;
  param_1[0x59] = *(int *)((int)this + 4);
  FUN_005b69b0((int)param_1);
  *(int *)((int)this + 8) = *(int *)((int)this + 8) + -1;
  *(int *)((int)this + 0xc) = *(int *)((int)this + 0xc) + -1;
  this_00 = (void *)FUN_005aaf90();
  FUN_005a9170(this_00,param_1);
  return;
}

