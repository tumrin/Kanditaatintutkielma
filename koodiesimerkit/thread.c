int counter;
pthread_mutex_t lock;

// This is not thread safe
void *increment_counter()
{
    printf("Before %d\n", counter);
    counter += 1;
    printf("After %d\n", counter);
    return NULL;
}

// This is thread safe
void *increment_counter_locked()
{
    pthread_mutex_lock(&lock);
    printf("Before %d\n", counter);
    counter += 1;
    printf("After %d\n", counter);
    pthread_mutex_unlock(&lock);
    return NULL;
}